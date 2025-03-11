use std::{collections::HashMap, sync::LazyLock};

use homm5_types::art::AdvMapArtifact;
use quick_xml::Writer;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display, EnumIter, Clone)]
pub enum NewArtifactType {
    #[strum(serialize = "ARTIFACT_PRIMARY_WEAPON_BLANK")]
    PrimaryWeaponBlank,
    #[strum(serialize = "ARTIFACT_SECONDARY_WEAPON_BLANK")]
    SecondaryWeaponBlank,
    #[strum(serialize = "ARTIFACT_ARMOR_BLANK")]
    ArmorBlank,
    #[strum(serialize = "ARTIFACT_HELMET_BLANK")]
    HelmetBlank,
    #[strum(serialize = "ARTIFACT_BOOTS_BLANK")]
    BootsBlank,
    #[strum(serialize = "ARTIFACT_RING_BLANK")]
    RingBlank,
    #[strum(serialize = "ARTIFACT_NECKLACE_BLANK")]
    NeclaceBlank,
    #[strum(serialize = "ARTIFACT_CLOAK_BLANK")]
    CloakBlank,
    #[strum(serialize = "ARTIFACT_POCKET_BLANK")]
    PocketBlank,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_POWER")]
    InsigniaOfPower,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_PROTECTION")]
    InsigniaOfProtection,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_WIZARDY")]
    InsigniaOfWizardy,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_ENLIGHTMENT")]
    InsigniaOfEnlightment,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_HASTE")]
    InsigniaOfHaste,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_SWIFTNESS")]
    InsigniaOfSwiftness,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_VITALITY")]
    InsigniaOfVitality,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_FORTUNE")]
    InsigniaOfFortune,
    #[strum(serialize = "ARTIFACT_INSIGNIA_OF_COURAGE")]
    InsigniaOfCourage,
    #[strum(serialize = "ARTIFACT_SEAL_OF_MIGHT")]
    SealOfMight,
    #[strum(serialize = "ARTIFACT_SEAL_OF_MAGIC")]
    SealOfMagic,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_STEPPE")]
    SigilOfSteppe,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_BLOOM")]
    SigilOfBloom,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_CALMNESS")]
    SigilOfCalmness,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_DEATH")]
    SigilOfDeath,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_BURNING_HEART")]
    SigilOfBurningHeart,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_LIGHT")]
    SigilOfLight,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_PROGRESS")]
    SigilOfProgress,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_RAGE")]
    SigilOfRage,
    #[strum(serialize = "ARTIFACT_SIGIL_OF_DRAGONS")]
    SigilOfDragons
}

static NEW_ARTIFACTS_XDBS: LazyLock<HashMap<NewArtifactType, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NewArtifactType::PrimaryWeaponBlank, "/MapObjects/Artifacts/NAF/ART_175/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SecondaryWeaponBlank, "/MapObjects/Artifacts/NAF/ART_176/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::ArmorBlank, "/MapObjects/Artifacts/NAF/ART_177/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::HelmetBlank, "/MapObjects/Artifacts/NAF/ART_178/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::BootsBlank, "/MapObjects/Artifacts/NAF/ART_179/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::RingBlank, "/MapObjects/Artifacts/NAF/ART_180/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::NeclaceBlank, "/MapObjects/Artifacts/NAF/ART_181/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::CloakBlank, "/MapObjects/Artifacts/NAF/ART_182/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::PocketBlank, "/MapObjects/Artifacts/NAF/ART_183/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfPower, "/MapObjects/Artifacts/NAF/ART_184/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfProtection, "/MapObjects/Artifacts/NAF/ART_185/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfWizardy, "/MapObjects/Artifacts/NAF/ART_186/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfEnlightment, "/MapObjects/Artifacts/NAF/ART_187/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfHaste, "/MapObjects/Artifacts/NAF/ART_188/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfSwiftness, "/MapObjects/Artifacts/NAF/ART_189/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfVitality, "/MapObjects/Artifacts/NAF/ART_190/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfFortune, "/MapObjects/Artifacts/NAF/ART_191/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::InsigniaOfCourage, "/MapObjects/Artifacts/NAF/ART_192/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SealOfMight, "/MapObjects/Artifacts/NAF/ART_193/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SealOfMagic, "/MapObjects/Artifacts/NAF/ART_194/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfSteppe, "/MapObjects/Artifacts/NAF/ART_195/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfBloom, "/MapObjects/Artifacts/NAF/ART_196/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfCalmness, "/MapObjects/Artifacts/NAF/ART_197/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfDeath, "/MapObjects/Artifacts/NAF/ART_198/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfBurningHeart, "/MapObjects/Artifacts/NAF/ART_199/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfLight, "/MapObjects/Artifacts/NAF/ART_200/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfProgress, "/MapObjects/Artifacts/NAF/ART_201/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfRage, "/MapObjects/Artifacts/NAF/ART_202/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)"),
        (NewArtifactType::SigilOfDragons, "/MapObjects/Artifacts/NAF/ART_203/Art.(AdvMapArtifactShared).xdb#xpointer(/AdvMapArtifactShared)")
    ])
});

pub struct ArtifactsModifier {
    new_artifacts: HashMap<NewArtifactType, Vec<String>>,
    new_artifacts_count: HashMap<NewArtifactType, u32>
}

impl ArtifactsModifier {
    pub fn new() -> Self {
        ArtifactsModifier {
            new_artifacts: HashMap::new(),
            new_artifacts_count: HashMap::new()
        }
    }

    pub fn modify(&mut self, artifact: &mut AdvMapArtifact, writer: &mut Writer<&mut Vec<u8>>) {
        let empty = String::new();
        let artifact_shared = artifact.shared.href.as_ref().unwrap_or(&empty);
        if !artifact_shared.is_empty() {
            if let Some(artifact_data) = NEW_ARTIFACTS_XDBS.iter().find(|(_type, shared)| **shared == artifact_shared.as_str()) {
                let artifacts_count;
                if let Some(current_count) = self.new_artifacts_count.get_mut(artifact_data.0) {
                    artifacts_count = *current_count + 1;
                } else {
                    artifacts_count = 1;
                }
                artifact.name = Some(format!("{}_{}", &artifact_data.0.to_string(), artifacts_count));
                self.new_artifacts_count.insert(artifact_data.0.clone(), artifacts_count);
                if let Some(existing_artifacts) = self.new_artifacts.get_mut(artifact_data.0) {
                    existing_artifacts.push(artifact.name.clone().unwrap());
                } else {
                    self.new_artifacts.insert(artifact_data.0.clone(), vec![artifact.name.clone().unwrap()]);
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
                    lua_code += &format!("\t[\"{}\"] = {},\n", artifact, _type.to_string());
                }
            }
        });
        lua_code.push_str("}\n\n");
        lua_code
    }
}