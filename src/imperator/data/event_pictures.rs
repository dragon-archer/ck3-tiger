use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct EventPicture {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Imperator, Item::EventPicture, EventPicture::add)
}

impl EventPicture {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::EventPicture, key, block, Box::new(Self {}));
    }
}

impl DbKind for EventPicture {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        let mut sc = ScopeContext::new(Scopes::Country, key);

        vd.field_item("theme", Item::EventTheme);
        vd.field_item("picture", Item::File);

        vd.field_validated_block("picture", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.field_item("texture", Item::File);
            vd.field_validated_block("trigger", |b, data| {
                validate_trigger(b, data, &mut sc, Tooltipped::No);
            });
        });
    }
}
