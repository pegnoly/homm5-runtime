use std::collections::HashMap;

use homm5_types::art::AdvMapArtifact;
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display, EnumIter, Clone, Serialize, Deserialize)]
pub enum NewArtifactType {
    #[serde(rename = "ARTIFACT_PRIMARY_WEAPON_BLANK")]
    #[strum(serialize = "ARTIFACT_PRIMARY_WEAPON_BLANK")]
    PrimaryWeaponBlank,
    #[serde(rename = "ARTIFACT_SECONDARY_WEAPON_BLANK")]
    #[strum(serialize = "ARTIFACT_SECONDARY_WEAPON_BLANK")]
    SecondaryWeaponBlank,
    #[serde(rename = "ARTIFACT_ARMOR_BLANK")]
    #[strum(serialize = "ARTIFACT_ARMOR_BLANK")]
    ArmorBlank,
    #[serde(rename = "ARTIFACT_HELMET_BLANK")]
    #[strum(serialize = "ARTIFACT_HELMET_BLANK")]
    HelmetBlank,
    #[serde(rename = "ARTIFACT_BOOTS_BLANK")]
    #[strum(serialize = "ARTIFACT_BOOTS_BLANK")]
    BootsBlank,
    #[serde(rename = "ARTIFACT_RING_BLANK")]
    #[strum(serialize = "ARTIFACT_RING_BLANK")]
    RingBlank,
    #[serde(rename = "ARTIFACT_NECKLACE_BLANK")]
    #[strum(serialize = "ARTIFACT_NECKLACE_BLANK")]
    NeclaceBlank,
    #[serde(rename = "ARTIFACT_CLOAK_BLANK")]
    #[strum(serialize = "ARTIFACT_CLOAK_BLANK")]
    CloakBlank,
    #[serde(rename = "ARTIFACT_POCKET_BLANK")]
    #[strum(serialize = "ARTIFACT_POCKET_BLANK")]
    PocketBlank,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_POWER")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_POWER")]
    InsigniaOfPower,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_PROTECTION")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_PROTECTION")]
    InsigniaOfProtection,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_WIZARDY")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_WIZARDY")]
    InsigniaOfWizardy,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_ENLIGHTMENT")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_ENLIGHTMENT")]
    InsigniaOfEnlightment,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_HASTE")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_HASTE")]
    InsigniaOfHaste,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_SWIFTNESS")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_SWIFTNESS")]
    InsigniaOfSwiftness,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_VITALITY")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_VITALITY")]
    InsigniaOfVitality,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_FORTUNE")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_FORTUNE")]
    InsigniaOfFortune,
    #[serde(rename = "ARTIFACT_INSIGNIA_OF_COURAGE")]
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_COURAGE")]
    InsigniaOfCourage,
    #[serde(rename = "ARTIFACT_SEAL_OF_MIGHT")]
    #[strum(serialize = "ARTIFACT_SEAL_OF_MIGHT")]
    SealOfMight,
    #[serde(rename = "ARTIFACT_SEAL_OF_MAGIC")]
    #[strum(serialize = "ARTIFACT_SEAL_OF_MAGIC")]
    SealOfMagic,
    #[serde(rename = "ARTIFACT_SIGIL_OF_STEPPE")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_STEPPE")]
    SigilOfSteppe,
    #[serde(rename = "ARTIFACT_SIGIL_OF_BLOOM")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_BLOOM")]
    SigilOfBloom,
    #[serde(rename = "ARTIFACT_SIGIL_OF_CALMNESS")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_CALMNESS")]
    SigilOfCalmness,
    #[serde(rename = "ARTIFACT_SIGIL_OF_DEATH")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_DEATH")]
    SigilOfDeath,
    #[serde(rename = "ARTIFACT_SIGIL_OF_BURNING_HEART")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_BURNING_HEART")]
    SigilOfBurningHeart,
    #[serde(rename = "ARTIFACT_SIGIL_OF_LIGHT")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_LIGHT")]
    SigilOfLight,
    #[serde(rename = "ARTIFACT_SIGIL_OF_PROGRESS")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_PROGRESS")]
    SigilOfProgress,
    #[serde(rename = "ARTIFACT_SIGIL_OF_RAGE")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_RAGE")]
    SigilOfRage,
    #[serde(rename = "ARTIFACT_SIGIL_OF_DRAGONS")]
    #[strum(serialize = "ARTIFACT_SIGIL_OF_DRAGONS")]
    SigilOfDragons
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactConfigEntity {
    #[serde(rename = "type")]
    pub _type: NewArtifactType,
    pub shared: String
}

pub struct ArtifactsModifier<'a> {
    artifacts_data: &'a Vec<ArtifactConfigEntity>,
    new_artifacts: HashMap<NewArtifactType, Vec<String>>
}

impl<'a> ArtifactsModifier<'a> {
    pub fn new(artifacts_config_data: &'a Vec<ArtifactConfigEntity>) -> Self {
        ArtifactsModifier {
            artifacts_data: artifacts_config_data,
            new_artifacts: HashMap::new()
        }
    }

    pub fn modify(&mut self, artifact: &mut AdvMapArtifact, writer: &mut Writer<&mut Vec<u8>>) {
        let empty = String::new();
        let artifact_shared = artifact.shared.href.as_ref().unwrap_or(&empty);
        if !artifact_shared.is_empty() {
            if let Some(artifact_data) = self.artifacts_data.iter().find(|a| a.shared == artifact_shared.as_str()) {
                if let Some(artifacts) = self.new_artifacts.get_mut(&artifact_data._type) {
                    artifact.name = Some(format!("{}_{}", artifact_data._type, artifacts.len() + 1));
                    artifacts.push(artifact.name.clone().unwrap());
                } else {
                    artifact.name = Some(format!("{}_1", artifact_data._type));
                    self.new_artifacts.insert(artifact_data._type.clone(), vec![artifact.name.clone().unwrap()]);
                }
            }
        }
        if artifact.point_lights.is_some() && artifact.point_lights.as_ref().unwrap().items.is_none() {
            artifact.point_lights = None;
        }
        if artifact.army_slots.is_some() && artifact.army_slots.as_ref().unwrap().army_slots.is_none() {
            artifact.army_slots = None;
        }
        writer.write_serializable("AdvMapArtifact", artifact).unwrap();
    }

    pub fn convert_to_lua(&self) -> String {
        let mut lua_code = String::from("NEW_ARTIFACTS_DATA = {\n");
        NewArtifactType::iter().for_each(|_type| {
            if let Some(artifacts) = self.new_artifacts.get(&_type) {
                for artifact in artifacts {
                    lua_code += &format!("\t[\"{artifact}\"] = {_type},\n");
                }
            }
        });
        lua_code.push_str("}\n\n");
        lua_code
    }
}