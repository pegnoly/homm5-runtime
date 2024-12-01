use serde::{Serialize, Deserialize};
use crate::common::FileRef;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PossessionMarkerTile {
    pub x: u8,
    pub y: u8
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Skill {
    pub Mastery: String,
    pub SkillID: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Editable {
    pub NameFileRef: Option<FileRef>,
    pub BiographyFileRef: Option<FileRef>,
    pub Offence: u16,
    pub Defence: u16,
    pub Spellpower: u16,
    pub Knowledge: u16,
    // #[serde(rename = "Item")]
    // pub skills: Option<Vec<Skill>>,
    // #[serde(rename = "Item")]
    // pub perkIDs: Option<Vec<String>>,
    // #[serde(rename = "Item")]
    // pub spellIDs: Option<Vec<String>>,
    pub Ballista: bool,
    pub FirstAidTent: bool,
    pub AmmoCart: bool,
    //pub FavoriteEnemies: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
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
    // pub FaceTextureSmall: Option<FileRef>,
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