use serde::{Serialize, Deserialize};
use crate::{common::{FileRef, Pos}, Homm5Type};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Spell {
    pub Spell: String,
    pub Mastery: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MagicElement {
    pub First: String,
    pub Second: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Resources {
    pub Wood: u16,
    pub Ore: u16,
    pub Mercury: u16,
    pub Crystal: u16,
    pub Sulfur: u16,
    pub Gem: u16,
    pub Gold: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Abilities {
    #[serde(rename = "Item")]
    pub Abilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnownSpells {
    #[serde(rename = "Item")]
    pub spells: Option<Vec<Spell>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CreatureVisual {
    pub CreatureNameFileRef: Option<FileRef>,
    pub DescriptionFileRef: Option<FileRef>,
    pub Icon128: Option<FileRef>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AdvMapCreatureShared {
    pub AttackSkill: u16,
    pub DefenceSkill: u16,
    pub MinDamage: u16,
    pub MaxDamage: u16,
    pub Speed: u8,
    pub Initiative: u16,
    pub Flying: bool,
    pub Health: u32,
    pub KnownSpells: KnownSpells,
    pub SpellPoints: u16,
    pub Exp: u64,
    pub Power: u64,
    pub CreatureTier: u8,
    pub Upgrade: bool,
    pub PairCreature: String,
    pub CreatureTown: String,
    pub MagicElement: MagicElement,
    pub WeeklyGrowth: u16,
    pub Cost: Resources,
    pub SubjectOfRandomGeneration: bool,
    pub MonsterShared: Option<FileRef>,
    pub CombatSize: u8,
    pub Visual: Option<FileRef>,
    pub Range: i8,
    pub BaseCreature: Option<String>,
    #[serde(rename = "Item")]
    pub Upgrades: Option<Vec<String>>,
    pub Abilities: Abilities,
    pub VisualExplained: Option<CreatureVisual>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stack {
    #[serde(rename = "Creature")]
    pub creature: String,
    #[serde(rename = "CustomAmount")]
    pub is_custom_amount: bool,
    #[serde(rename = "Amount")]
    pub min_count: u32,
    #[serde(rename = "Amount2")]
    pub max_count: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalStacks {
    #[serde(rename = "Item")]
    pub items: Option<Vec<Stack>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvMapMonster {
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
    pub shared: Option<FileRef>,
    #[serde(rename = "Custom")]
    pub is_custom_amount: bool,
    #[serde(rename = "Amount")]
    pub min_count: u32,
    #[serde(rename = "Amount2")]
    pub max_count: u32,
    #[serde(rename = "AttackType")]
    pub attack_type: String,
    #[serde(rename = "MoveType")]
    pub move_type: String,
    #[serde(rename = "DoesNotGrow")]
    pub does_not_grow: bool,
    #[serde(rename = "MessageFileRef")]
    pub message_file_ref: Option<FileRef>,
    #[serde(rename = "Script")]
    pub script: Option<String>,
    #[serde(rename = "Resources")]
    pub resources: Resources,
    #[serde(rename = "ArtifactID")]
    pub art_id: String,
    #[serde(rename = "Mood")]
    pub mood: String,
    #[serde(rename = "Courage")]
    pub courage: String,
    #[serde(rename = "AllowQuickCombat")]
    pub allow_quick_combat: bool,
    #[serde(rename = "DoesNotDependOnDifficulty")]
    pub does_not_depends_on_difficulty: bool,
    #[serde(rename = "AdditionalStacks")]
    pub additional_stacks: Option<AdditionalStacks>,
    #[serde(rename = "SingleMonsterNameFileRef")]
    pub single_monster_name: Option<FileRef>,
    #[serde(rename = "MultipleMonstersNameFileRef")]
    pub multiple_monster_name: Option<FileRef>,
    #[serde(rename = "RacesRandomGroupID")]
    pub race_random_group_id: u32,
    #[serde(rename = "relationsOverrides")]
    pub relations_override: Option<String>
}

impl Homm5Type for AdvMapMonster {
}