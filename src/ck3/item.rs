use strum_macros::{EnumIter, IntoStaticStr};

use crate::report::{Confidence, Severity};

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoStaticStr, Hash, PartialOrd, Ord, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Item {
    Accessory,
    AccessoryTag,
    AccessoryVariation,
    AccessoryVariationLayout,
    AccessoryVariationTextures,
    AccoladeCategory,
    AccoladeIcon,
    AccoladeName,
    AccoladeParameter,
    AccoladeType,
    ActivityIntent,
    ActivityLocale,
    ActivityOption,
    ActivityOptionCategory,
    ActivityPhase,
    ActivityState,
    ActivityType,
    Amenity,
    ArtifactFeature,
    ArtifactFeatureGroup,
    ArtifactHistory,
    ArtifactRarity,
    ArtifactSlot,
    ArtifactSlotType,
    ArtifactTemplate,
    ArtifactType,
    ArtifactVisual,
    Asset,
    BlendShape,
    Bookmark,
    BookmarkGroup,
    BookmarkPortrait,
    Building,
    BuildingFlag,
    BuildingGfx,
    CasusBelli,
    CasusBelliGroup,
    Catalyst,
    Character,
    CharacterBackground,
    CharacterTemplate,
    ClothingGfx,
    Coa,
    CoaGfx,
    CoaColorList,
    CoaColoredEmblemList,
    CoaDesignerColoredEmblem,
    CoaDesignerColorPalette,
    CoaDesignerEmblemLayout,
    CoaDesignerPattern,
    CoaDynamicDefinition,
    CoaPatternList,
    CoaTemplate,
    CoaTemplateList,
    CoaTexturedEmblemList,
    CombatEffect,
    CombatPhaseEvent,
    CouncilPosition,
    CouncilTask,
    CourtPosition,
    CourtPositionCategory,
    CourtSceneCulture,
    CourtSceneGroup,
    CourtSceneRole,
    CourtSceneSetting,
    CourtType,
    Culture,
    CultureAesthetic,
    CultureCreationName,
    CultureEra,
    CultureEthos,
    CultureHeritage,
    CultureHistory,
    CultureParameter,
    CulturePillar,
    CultureTradition,
    CustomLocalization,
    DangerType,
    DeathReason,
    Decision,
    Define,
    DiarchyMandate,
    DiarchyParameter,
    DiarchyType,
    Dlc,
    DlcFeature,
    Dna,
    Doctrine,
    DoctrineParameter,
    Dynasty,
    DynastyLegacy,
    DynastyPerk,
    EffectLocalization,
    Entity,
    Environment,
    Ethnicity,
    Event,
    EventBackground,
    EventNamespace,
    EventTheme,
    EventTransition,
    Faction,
    Faith,
    FaithIcon,
    File,
    Flavorization,
    Focus,
    GameConcept,
    GameRule,
    GameRuleSetting,
    GeneAgePreset,
    GeneAttribute,
    GeneCategory,
    GeneticConstraint,
    GovernmentType,
    GovernmentFlag,
    GraphicalFaith,
    GuestInviteRule,
    GuestSubset,
    Holding,
    HoldingFlag,
    HolySite,
    HolySiteFlag,
    Hook,
    House,
    ImportantAction,
    Innovation,
    InnovationFlag,
    Inspiration,
    Interaction,
    InteractionCategory,
    Language,
    Law,
    LawFlag,
    LawGroup,
    Lifestyle,
    Localization,
    MapEnvironment,
    MapMode,
    MartialCustom,
    MemoryCategory,
    MemoryType,
    MenAtArms,
    MenAtArmsBase,
    Message,
    Modifier,
    ModifierFormat,
    MottoInsert,
    Motto,
    Music,
    NamedColor,
    NameList,
    Nickname,
    OnAction,
    OpinionModifier,
    Pdxmesh,
    Perk,
    PerkTree,
    PlayableDifficultyInfo,
    PointOfInterest,
    PoolSelector,
    PortraitAnimation,
    PortraitCamera,
    PortraitModifierGroup,
    PortraitModifierPack,
    PrisonType,
    Province,
    PulseAction,
    Region,
    Relation,
    RelationFlag,
    Religion,
    ReligionFamily,
    RewardItem,
    Scheme,
    ScriptedAnimation,
    ScriptedCost,
    ScriptedEffect,
    ScriptedGui,
    ScriptedIllustration,
    ScriptedList,
    ScriptedModifier,
    ScriptedRule,
    ScriptedTrigger,
    ScriptValue,
    Secret,
    Sexuality,
    Skill,
    Sound,
    SpecialBuilding,
    SpecialGuest,
    Story,
    Struggle,
    StrugglePhase,
    StrugglePhaseParameter,
    SuccessionElection,
    Suggestion,
    Terrain,
    TextureFile,
    Title,
    TitleHistory,
    TitleHistoryType,
    Trait,
    TraitCategory,
    TraitFlag,
    TraitTrack,
    TravelOption,
    TriggerLocalization,
    UnitGfx,
    VassalContract,
    VassalContractFlag,
    VassalObligationLevel,
    VassalStance,
}

impl Item {
    pub fn path(self) -> &'static str {
        #[allow(clippy::match_same_arms)]
        match self {
            Item::Accessory => "gfx/portraits/accessories/",
            Item::AccessoryTag => "gfx/portraits/accessories/",
            Item::AccessoryVariation => "gfx/portraits/accessory_variations/",
            Item::AccessoryVariationLayout => "gfx/portraits/accessory_variations/",
            Item::AccessoryVariationTextures => "gfx/portraits/accessory_variations/",
            Item::AccoladeCategory => "common/accolade_types/",
            Item::AccoladeIcon => "common/accolade_icons/",
            Item::AccoladeName => "common/accolade_names/",
            Item::AccoladeParameter => "common/accolade_types/",
            Item::AccoladeType => "common/accolade_types/",
            Item::ActivityIntent => "common/activities/intents/",
            Item::ActivityLocale => "common/activities/activity_locales/",
            Item::ActivityOption => "common/activities/activity_types/",
            Item::ActivityOptionCategory => "common/activities/activity_types/",
            Item::ActivityPhase => "common/activities/activity_types/",
            Item::ActivityState => "",
            Item::ActivityType => "common/activities/activity_types/",
            Item::Amenity => "common/court_amenities/",
            Item::ArtifactFeature => "common/artifacts/features/",
            Item::ArtifactFeatureGroup => "common/artifacts/feature_groups/",
            Item::ArtifactHistory => "",
            Item::ArtifactRarity => "",
            Item::ArtifactSlot => "common/artifacts/slots/",
            Item::ArtifactSlotType => "common/artifacts/slots/",
            Item::ArtifactTemplate => "common/artifacts/templates/",
            Item::ArtifactType => "common/artifacts/types/",
            Item::ArtifactVisual => "common/artifacts/visuals/",
            Item::Asset => "gfx/models/",
            Item::BlendShape => "gfx/models/",
            Item::Bookmark => "common/bookmarks/bookmarks/",
            Item::BookmarkGroup => "common/bookmarks/groups/",
            Item::BookmarkPortrait => "common/bookmark_portraits/",
            Item::Building => "common/buildings/",
            Item::BuildingFlag => "common/buildings/",
            Item::BuildingGfx => "common/culture/cultures/",
            Item::CasusBelli => "common/casus_belli_types/",
            Item::CasusBelliGroup => "common/casus_belli_groups/",
            Item::Catalyst => "common/struggle/catalysts/",
            Item::Character => "history/characters/",
            Item::CharacterBackground => "common/character_backgrounds/",
            Item::CharacterTemplate => "common/scripted_character_templates/",
            Item::ClothingGfx => "common/culture/cultures/",
            Item::Coa => "common/coat_of_arms/coat_of_arms/",
            Item::CoaGfx => "common/culture/cultures/",
            Item::CoaColorList => "common/coat_of_arms/template_lists/",
            Item::CoaColoredEmblemList => "common/coat_of_arms/template_lists/",
            Item::CoaDesignerColoredEmblem => "gfx/coat_of_arms/colored_emblems/",
            Item::CoaDesignerColorPalette => "gfx/coat_of_arms/color_palettes/",
            Item::CoaDesignerEmblemLayout => "gfx/coat_of_arms/emblem_layouts/",
            Item::CoaDesignerPattern => "gfx/coat_of_arms/patterns/",
            Item::CoaDynamicDefinition => "common/coat_of_arms/dynamic_definitions/",
            Item::CoaPatternList => "common/coat_of_arms/template_lists/",
            Item::CoaTemplate => "common/coat_of_arms/coat_of_arms/",
            Item::CoaTemplateList => "common/coat_of_arms/template_lists/",
            Item::CoaTexturedEmblemList => "common/coat_of_arms/template_lists/",
            Item::CombatEffect => "common/combat_effects/",
            Item::CombatPhaseEvent => "common/combat_phase_events/",
            Item::CouncilPosition => "common/council_positions/",
            Item::CouncilTask => "common/council_tasks/",
            Item::CourtPosition => "common/court_positions/types/",
            Item::CourtPositionCategory => "common/court_positions/categories/",
            Item::CourtSceneCulture => "gfx/court_scene/scene_cultures/",
            Item::CourtSceneGroup => "gfx/court_scene/character_groups/",
            Item::CourtSceneRole => "gfx/court_scene/character_roles/",
            Item::CourtSceneSetting => "gfx/court_scene/scene_settings/",
            Item::CourtType => "common/court_types/",
            Item::Culture => "common/culture/cultures/",
            Item::CultureAesthetic => "common/culture/aesthetics_bundles/",
            Item::CultureCreationName => "common/culture/creation_names/",
            Item::CultureEra => "common/culture/eras/",
            Item::CultureEthos => "common/culture/pillars/",
            Item::CultureHeritage => "common/culture/pillars/",
            Item::CultureHistory => "history/cultures/",
            Item::CultureParameter => "common/culture/",
            Item::CulturePillar => "common/culture/pillars/",
            Item::CultureTradition => "common/culture/traditions/",
            Item::CustomLocalization => "common/customizable_localization/",
            Item::DangerType => "",
            Item::DeathReason => "common/deathreasons/",
            Item::Decision => "common/decisions/",
            Item::Define => "common/defines/",
            Item::DiarchyMandate => "common/diarchies/diarchy_mandates/",
            Item::DiarchyParameter => "common/diarchies/diarchy_types/",
            Item::DiarchyType => "common/diarchies/diarchy_types/",
            Item::Dlc => "",
            Item::DlcFeature => "",
            Item::Dna => "common/dna_data/",
            Item::Doctrine => "common/religion/doctrines/",
            Item::DoctrineParameter => "common/religion/doctrines/",
            Item::Dynasty => "common/dynasties/",
            Item::DynastyLegacy => "common/dynasty_legacies/",
            Item::DynastyPerk => "common/dynasty_perks/",
            Item::EffectLocalization => "common/effect_localization/",
            Item::Ethnicity => "common/ethnicities/",
            Item::Entity => "gfx/models/",
            Item::Environment => "gfx/portraits/environments/",
            Item::Event => "events/",
            Item::EventBackground => "common/event_backgrounds/",
            Item::EventNamespace => "events/",
            Item::EventTheme => "common/event_themes/",
            Item::EventTransition => "common/event_transitions/",
            Item::Faith => "common/religion/religions/",
            Item::FaithIcon => "common/religion/religions/",
            Item::Faction => "common/factions/",
            Item::File => "",
            Item::Flavorization => "common/flavorization/",
            Item::Focus => "common/focuses/",
            Item::GameConcept => "common/game_concepts/",
            Item::GameRule => "common/game_rules/",
            Item::GameRuleSetting => "common/game_rules/",
            Item::GeneAgePreset => "common/genes/",
            Item::GeneAttribute => "gfx/models/",
            Item::GeneCategory => "common/genes/",
            Item::GeneticConstraint => "common/traits/",
            Item::GovernmentType => "common/governments/",
            Item::GovernmentFlag => "common/governments/",
            Item::GraphicalFaith => "common/religion/religions/",
            Item::GuestInviteRule => "common/activities/guest_invite_rules/",
            Item::GuestSubset => "common/activities/activity_types/",
            Item::Holding => "common/holdings/",
            Item::HoldingFlag => "common/holdings/",
            Item::HolySite => "common/religion/holy_sites/",
            Item::HolySiteFlag => "common/religion/holy_sites/",
            Item::Hook => "common/hook_types/",
            Item::House => "common/dynasty_houses/",
            Item::ImportantAction => "common/important_actions/",
            Item::Innovation => "common/culture/innovations/",
            Item::InnovationFlag => "common/culture/innovations/",
            Item::Inspiration => "common/inspirations/",
            Item::Interaction => "common/character_interactions/",
            Item::InteractionCategory => "common/character_interaction_categories/",
            Item::Language => "common/culture/pillars/",
            Item::Law => "common/laws/",
            Item::LawFlag => "common/laws/",
            Item::LawGroup => "common/laws/",
            Item::Lifestyle => "common/lifestyles/",
            Item::Localization => "localization/",
            Item::MapEnvironment => "gfx/map/environment/",
            Item::MapMode => "gfx/map/map_modes/",
            Item::MartialCustom => "common/culture/pillars/",
            Item::MemoryCategory => "common/character_memory_types/",
            Item::MemoryType => "common/character_memory_types/",
            Item::MenAtArms => "common/men_at_arms_types/",
            Item::MenAtArmsBase => "common/men_at_arms_types/",
            Item::Message => "common/messages",
            Item::Modifier => "common/modifiers/",
            Item::ModifierFormat => "common/modifier_definition_formats/",
            Item::MottoInsert => "common/dynasty_house_motto_inserts/",
            Item::Motto => "common/dynasty_house_mottos/",
            Item::Music => "music/",
            Item::NamedColor => "common/named_colors/",
            Item::NameList => "common/culture/name_lists/",
            Item::Nickname => "common/nicknames/",
            Item::OnAction => "common/on_action/",
            Item::OpinionModifier => "common/opinion_modifiers/",
            Item::Pdxmesh => "gfx/models/",
            Item::Perk => "common/lifestyle_perks/",
            Item::PerkTree => "common/lifestyle_perks/",
            Item::PlayableDifficultyInfo => "common/playable_difficulty_infos/",
            Item::PointOfInterest => "common/travel/point_of_interest_types/",
            Item::PoolSelector => "common/pool_character_selectors/",
            Item::PortraitAnimation => "gfx/portraits/portrait_animations/",
            Item::PortraitCamera => "gfx/portraits/cameras/",
            Item::PortraitModifierGroup => "gfx/portraits/portrait_modifiers/",
            Item::PortraitModifierPack => "gfx/portraits/portrait_animations/",
            Item::PrisonType => "",
            Item::Province => "map_data/definition.csv",
            Item::PulseAction => "common/activities/pulse_actions/",
            Item::Region => "map_data/geographical_regions/",
            Item::Relation => "common/scripted_relations/",
            Item::RelationFlag => "common/scripted_relations/",
            Item::Religion => "common/religion/religions/",
            Item::ReligionFamily => "common/religion/religion_families/",
            Item::RewardItem => "",
            Item::Scheme => "common/schemes/",
            Item::ScriptedAnimation => "common/scripted_animations/",
            Item::ScriptedCost => "common/scripted_costs/",
            Item::ScriptedEffect => "common/scripted_effects/",
            Item::ScriptedGui => "common/scripted_guis/",
            Item::ScriptedIllustration => "gfx/interface/illustrations/scripted_illustrations/",
            Item::ScriptedList => "common/scripted_lists/",
            Item::ScriptedModifier => "common/scripted_modifiers/",
            Item::ScriptedRule => "common/scripted_rules/",
            Item::ScriptedTrigger => "common/scripted_triggers/",
            Item::ScriptValue => "common/script_values/",
            Item::Secret => "common/secret_types/",
            Item::Sexuality => "",
            Item::Skill => "",
            Item::Sound => "sound/GUIDs.txt",
            Item::SpecialBuilding => "common/buildings/",
            Item::SpecialGuest => "common/activities/activity_types/",
            Item::Story => "common/story_cycles/",
            Item::Struggle => "common/struggle/struggles/",
            Item::StrugglePhase => "common/struggle/struggles/",
            Item::StrugglePhaseParameter => "common/struggle/struggles/",
            Item::SuccessionElection => "common/succession_election/",
            Item::Suggestion => "common/suggestions/",
            Item::Terrain => "common/terrain_types/",
            Item::TextureFile => "gfx/models/",
            Item::Title => "common/landed_titles/",
            Item::TitleHistory => "history/titles/",
            Item::TitleHistoryType => "",
            Item::Trait => "common/traits/",
            Item::TraitCategory => "",
            Item::TraitFlag => "common/traits/",
            Item::TraitTrack => "common/traits/",
            Item::TravelOption => "common/travel/travel_options/",
            Item::TriggerLocalization => "common/trigger_localization/",
            Item::UnitGfx => "common/culture/cultures/",
            Item::VassalContract => "common/vassal_contracts/",
            Item::VassalContractFlag => "common/vassal_contracts/",
            Item::VassalObligationLevel => "common/vassal_contracts/",
            Item::VassalStance => "common/vassal_stances/",
        }
    }

    /// Confidence value to use when reporting that an item is missing.
    /// Should be `Strong` for most, `Weak` for items that aren't defined anywhere but just used (such as gfx flags).
    pub fn confidence(self) -> Confidence {
        match self {
            Item::AccessoryTag
            | Item::AccoladeCategory
            | Item::BuildingGfx
            | Item::ClothingGfx
            | Item::CoaGfx
            | Item::MemoryCategory
            | Item::Sound
            | Item::UnitGfx => Confidence::Weak,
            Item::SpecialBuilding => Confidence::Reasonable,
            _ => Confidence::Strong,
        }
    }

    /// Severity value to use when reporting that an item is missing.
    /// * `Error` - most things
    /// * `Warning` - things that only impact visuals or presentation
    /// * `Untidy` - things that don't matter much at all
    /// * `Fatal` - things that cause crashes if they're missing
    /// This is only one piece of the severity puzzle. It can also depend on the caller who's expecting the item to exist.
    /// That part isn't handled yet.
    pub fn severity(self) -> Severity {
        match self {
            Item::Accessory
            | Item::AccessoryTag
            | Item::AccessoryVariation
            | Item::AccessoryVariationLayout
            | Item::AccessoryVariationTextures
            | Item::AccoladeIcon
            | Item::ArtifactVisual
            | Item::BuildingGfx
            | Item::ClothingGfx
            | Item::Coa
            | Item::CoaGfx
            | Item::CoaColorList
            | Item::CoaColoredEmblemList
            | Item::CoaDynamicDefinition
            | Item::CoaPatternList
            | Item::CoaTemplate
            | Item::CoaTemplateList
            | Item::CoaTexturedEmblemList
            | Item::CultureAesthetic
            | Item::CultureCreationName
            | Item::CustomLocalization
            | Item::EffectLocalization
            | Item::Environment
            | Item::Ethnicity
            | Item::EventBackground
            | Item::EventTheme
            | Item::EventTransition
            | Item::Flavorization
            | Item::GameConcept
            | Item::GraphicalFaith
            | Item::Localization
            | Item::MapEnvironment
            | Item::ModifierFormat
            | Item::MottoInsert
            | Item::Motto
            | Item::Music
            | Item::NamedColor
            | Item::Nickname
            | Item::PortraitAnimation
            | Item::PortraitCamera
            | Item::ScriptedIllustration
            | Item::Sound
            | Item::TextureFile
            | Item::TriggerLocalization
            | Item::UnitGfx => Severity::Warning,
            _ => Severity::Error,
        }
    }
}
