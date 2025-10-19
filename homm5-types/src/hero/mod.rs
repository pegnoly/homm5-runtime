use crate::{common::{FileRef, Pos, Trigger}, player::PlayerID, town::ArmySlots};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PossessionMarkerTile {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Skill {
    pub Mastery: String,
    pub SkillID: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skills {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Skill>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Perks {
    #[serde(rename = "Item")]
    pub items: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellIds {
    #[serde(rename = "Item")]
    pub items: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FavoriteEnemies {
    #[serde(rename = "Item")]
    pub items: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(non_snake_case)]
pub struct Editable {
    pub NameFileRef: Option<FileRef>,
    pub BiographyFileRef: Option<FileRef>,
    pub Offence: u16,
    pub Defence: u16,
    pub Spellpower: u16,
    pub Knowledge: u16,
    pub skills: Option<Skills>,
    pub perkIDs: Option<Perks>,
    pub spellIDs: Option<SpellIds>,
    pub Ballista: bool,
    pub FirstAidTent: bool,
    pub AmmoCart: bool,
    pub FavoriteEnemies: Option<FavoriteEnemies>,
    pub TalismanLevel: Option<u8>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Textures {
    #[serde(rename = "Icon128x128")]
    pub icon128: Option<String>,
    #[serde(rename = "Icon64x64")]
    pub icon64: Option<String>,
    #[serde(rename = "RoundedFace")]
    pub rounded_face: Option<String>,
    #[serde(rename = "LeftFace")]
    pub left_face: Option<String>,
    #[serde(rename = "RightFace")]
    pub right_face: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct AdvMapHeroShared {
    pub Model: Option<FileRef>,
    pub AnimSet: Option<FileRef>,
    // #[serde(rename = "Item")]
    // pub blockedTiles: Option<Vec<Coordinate>>,
    // #[serde(rename = "Item")]
    // pub holeTiles: Option<Vec<Coordinate>>,
    // #[serde(rename = "Item")]
    // pub activeTiles: Option<Vec<Coordinate>>,
    // #[serde(rename = "Item")]
    // pub passableTiles: Option<Vec<Coordinate>>,
    // pub PossessionMarkerTile: Option<PossessionMarkerTile>,
    // pub Effect: Option<FileRef>,
    // pub EffectWhenOwned: Option<FileRef>,
    // pub messageFileRef: Option<FileRef>,
    pub WaterBased: bool,
    pub ApplyHeroTrace: bool,
    // pub SoundEffect: Option<FileRef>,
    // pub flybyMessageFileRef: Option<FileRef>,
    // pub ObjectTypeFileRef: Option<FileRef>,
    pub FlyPassable: bool,
    //pub AdventureSoundEffect: Option<FileRef>,
    //pub RazedStatic: Option<FileRef>,
    pub Icon128: Option<String>,
    pub InternalName: String,
    pub Class: String,
    pub Specialization: String,
    pub PrimarySkill: Skill,
    pub SpecializationNameFileRef: Option<FileRef>,
    pub SpecializationDescFileRef: Option<FileRef>,
    pub SpecializationIcon: Option<FileRef>,
    pub FaceTexture: Option<FileRef>,
    pub FaceTextureSmall: Option<FileRef>,
    // pub HeroCharacterArena: Option<FileRef>,
    // pub HeroCharacterArenaMelee: Option<String>,
    // pub HeroCharacterAdventure: Option<FileRef>,
    // pub HeroIndividualCamera: Option<FileRef>,
    // pub CombatVisual: Option<FileRef>,
    pub TownType: String,
    pub Editable: Editable,
    // pub ArrowButtonState: u8,
    // pub Decal: Option<FileRef>,
    // pub Selection: Option<FileRef>,
    // pub Trace: Option<FileRef>,
    // pub RaceTraitIcon: Option<FileRef>,
    // pub RaceTraitDescFileRef: Option<FileRef>,
    pub ScenarioHero: bool,
    // pub AdventureMusic: Option<FileRef>,
    // pub HideInEditor: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactIds {
    #[serde(rename = "Item")]
    pub items: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IsUntransferable {
    #[serde(rename = "Item")]
    pub items: Option<Vec<u8>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AdvMapHero {
    #[serde(rename = "Pos")]
    pub pos: Pos,
    #[serde(rename = "Rot")]
    pub rot: f32,
    #[serde(rename = "Floor")]
    pub floor: u8,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CombatScript")]
    pub combat_script: Option<String>,
    #[serde(rename = "pointLights")]
    pub point_lights: Option<String>,
    #[serde(rename = "Shared")]
    pub shared: FileRef,
    #[serde(rename = "PlayerID")]
    pub player_id: PlayerID,
    #[serde(rename = "Experience")]
    pub experience: u32,
    #[serde(rename = "armySlots")]
    pub army_slots: Option<ArmySlots>,
    #[serde(rename = "artifactIDs")]
    pub artifact_ids: Option<ArtifactIds>,
    #[serde(rename = "isUntransferable")]
    pub is_untransferable: Option<IsUntransferable>,
    #[serde(rename = "Editable")]
    pub editable: Editable,
    #[serde(rename = "OverrideMask")]
    pub override_mask: u16,
    #[serde(rename = "PrimarySkillMastery")]
    pub primary_skill_mastery: String,
    #[serde(rename = "LossTrigger")]
    pub loss_trigger: Trigger,
    #[serde(rename = "AllowQuickCombat")]
    pub allow_quick_combat: bool,
    #[serde(rename = "Textures")]
    pub textures: Textures,
    #[serde(rename = "PresetPrice")]
    pub preset_price: u16,
    #[serde(rename = "BannedRaces")]
    pub banned_races: Option<String>
}