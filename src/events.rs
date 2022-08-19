use fnv::FnvHashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::errors::{error, error_info, warn_info, ErrorKey, LogPauseRaii};
use crate::everything::{FileEntry, FileHandler, FileKind};
use crate::pdxfile::PdxFile;
use crate::scope::{Comparator, Loc, Scope, ScopeOrValue, Token};

#[derive(Clone, Debug, Default)]
pub struct Events {
    events: FnvHashMap<String, Event>,
    scripted_triggers: FnvHashMap<String, ScriptedTrigger>,
    scripted_effects: FnvHashMap<String, ScriptedEffect>,

    // These events are known to exist, so don't warn abour them not being found,
    // but they had errors on validation.
    error_events: FnvHashMap<String, Token>,
}

impl Events {
    pub fn load_event(&mut self, key: Token, scope: &Scope) {}
    pub fn load_scripted_trigger(&mut self, key: Token, scope: &Scope) {}
    pub fn load_scripted_effect(&mut self, key: Token, scope: &Scope) {}
}

impl FileHandler for Events {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("events")
    }

    fn config(&mut self, _config: &Scope) {}

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        #[derive(Copy, Clone)]
        enum Expecting {
            Event,
            ScriptedTrigger,
            ScriptedEffect,
        }

        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        // let _pause = LogPauseRaii::new(entry.kind() != FileKind::ModFile);

        let scope = match PdxFile::read(entry.path(), entry.kind(), fullpath, true) {
            Ok(scope) => scope,
            Err(e) => {
                let t = Token::from(Loc::for_file(
                    Rc::new(entry.path().to_path_buf()),
                    entry.kind(),
                ));
                error_info(
                    &t,
                    ErrorKey::ReadError,
                    "could not read file",
                    &format!("{:#}", e),
                );
                return;
            }
        };

        let mut namespace = None;
        let mut expecting = Expecting::Event;

        for (k, cmp, v) in scope.iter_items() {
            if let Some(key) = k {
                if !matches!(*cmp, Comparator::Eq) {
                    error(
                        key,
                        ErrorKey::Validation,
                        &format!("expected `{} =`, found `{}`", key, cmp),
                    );
                }
                if key.as_str() == "namespace" {
                    match v {
                        ScopeOrValue::Token(t) => namespace = Some(t.as_str()),
                        ScopeOrValue::Scope(s) => error(
                            &s.token(),
                            ErrorKey::EventNamespace,
                            "expected namespace to have a simple string value",
                        ),
                    }
                } else {
                    match v {
                        ScopeOrValue::Token(_) => error(
                            &key,
                            ErrorKey::Validation,
                            "unknown setting in event files, expected only `namespace`",
                        ),
                        ScopeOrValue::Scope(s) => match expecting {
                            Expecting::ScriptedTrigger => {
                                self.load_scripted_trigger(key.clone(), s);
                                expecting = Expecting::Event;
                            }
                            Expecting::ScriptedEffect => {
                                self.load_scripted_effect(key.clone(), s);
                                expecting = Expecting::Event;
                            }
                            Expecting::Event => {
                                let mut namespace_ok = false;

                                if let Some(namespace) = namespace {
                                    if let Some(key_a) = key.as_str().strip_prefix(namespace) {
                                        if let Some(key_b) = key_a.strip_prefix('.') {
                                            if key_b.chars().all(|c| c.is_ascii_digit()) {
                                                namespace_ok = true;
                                            } else {
                                                warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
                                            }
                                        } else {
                                            warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
                                        }
                                    } else {
                                        warn_info(key, ErrorKey::EventNamespace, "Event name should start with namespace", "If the event doesn't match its namespace, the game can't properly find the event when triggering it.")
                                    }
                                } else {
                                    error(
                                        key,
                                        ErrorKey::EventNamespace,
                                        "Event files must start with a namespace declaration",
                                    );
                                }

                                if namespace_ok {
                                    self.load_event(key.clone(), s);
                                } else {
                                    self.error_events
                                        .insert(key.as_str().to_string(), key.clone());
                                }
                            }
                        },
                    }
                }
            } else {
                match v {
                    ScopeOrValue::Token(t) => {
                        if matches!(expecting, Expecting::Event) && t.as_str() == "scripted_trigger"
                        {
                            expecting = Expecting::ScriptedTrigger;
                        } else if matches!(expecting, Expecting::Event)
                            && t.as_str() == "scripted_effect"
                        {
                            expecting = Expecting::ScriptedEffect;
                        } else {
                            error_info(
                                &t,
                                ErrorKey::Validation,
                                "unexpected token",
                                "Did you forget an = ?",
                            );
                        }
                    }
                    ScopeOrValue::Scope(s) => error_info(
                        &s.token(),
                        ErrorKey::Validation,
                        "unexpected block",
                        "Did you forget an = ?",
                    ),
                }
            }
        }
    }

    fn finalize(&mut self) {}
}

#[derive(Clone, Debug)]
pub struct Event {}

#[derive(Clone, Debug)]
pub struct ScriptedTrigger {}

#[derive(Clone, Debug)]
pub struct ScriptedEffect {}