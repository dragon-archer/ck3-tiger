use fnv::FnvHashMap;
use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use crate::errorkey::ErrorKey;
use crate::errors::{error_info, warn};
use crate::fileset::{FileEntry, FileHandler};
use crate::token::Token;

const DDS_HEADER_SIZE: usize = 124;
const DDS_HEIGHT_OFFSET: usize = 12;
const DDS_WIDTH_OFFSET: usize = 16;

fn from_le32(buffer: &[u8], offset: usize) -> u32 {
    buffer[offset] as u32
        | ((buffer[offset + 1] as u32) << 8)
        | ((buffer[offset + 2] as u32) << 16)
        | ((buffer[offset + 3] as u32) << 24)
}

#[derive(Clone, Debug, Default)]
pub struct DdsFiles {
    dds_files: FnvHashMap<String, DdsInfo>,
}

impl DdsFiles {
    fn load_dds(&mut self, entry: &FileEntry, fullpath: &Path) -> Result<()> {
        let mut f = File::open(fullpath)?;
        let mut buffer = [0; DDS_HEADER_SIZE];
        f.read_exact(&mut buffer)?;
        if !buffer.starts_with(b"DDS ") {
            warn(entry, ErrorKey::ImageFormat, "not a DDS file");
            return Ok(());
        }
        self.dds_files.insert(
            entry.path().to_string_lossy().to_string(),
            DdsInfo::new(&buffer),
        );
        Ok(())
    }

    pub fn validate_frame(&self, key: &Token, width: u32, height: u32, frame: u32) {
        if let Some(info) = self.dds_files.get(key.as_str()) {
            if info.height != height {
                let msg = format!("texture does not match frame height of {height}");
                warn(key, ErrorKey::ImageFormat, &msg);
                return;
            }
            if width == 0 || (info.width % width) != 0 {
                let msg = format!("texture is not a multiple of frame width {width}");
                warn(key, ErrorKey::ImageFormat, &msg);
                return;
            }
            // `frame` is 1-based
            if frame * width > info.width {
                let msg = format!("texture is not large enough for frame index {frame}");
                warn(key, ErrorKey::ImageFormat, &msg);
                return;
            }
        }
    }
}

impl FileHandler for DdsFiles {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("gfx")
    }

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        if !entry.filename().to_string_lossy().ends_with(".dds") {
            return;
        }

        match self.load_dds(entry, fullpath) {
            Ok(()) => (),
            Err(e) => {
                error_info(
                    entry,
                    ErrorKey::ReadError,
                    "could not read dds header",
                    &format!("{e:#}"),
                );
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DdsInfo {
    width: u32,
    height: u32,
}

impl DdsInfo {
    pub fn new(header: &[u8]) -> Self {
        let height = from_le32(&header, DDS_HEIGHT_OFFSET);
        let width = from_le32(&header, DDS_WIDTH_OFFSET);
        Self { width, height }
    }
}