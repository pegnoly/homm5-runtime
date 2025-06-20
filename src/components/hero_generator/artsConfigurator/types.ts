export enum ArtifactSlotType {
    Primary = "PRIMARY",
    Secondary = "SECONDARY",
    Head = "HEAD",
    Miscslot1 = "MISCSLOT1",
    Chest = "CHEST",
    Finger = "FINGER",
    Neck = "NECK",
    Feet = "FEET", 
    Shoulders = "SHOULDERS",
    Inventory = "INVENTORY"
}

export type DifficultyMappedValue = {
    data: Record<DifficultyType, number>
}

export type RequiredArtifacts = {
    ids: number[]
}

export type OptionalArtifacts = {
    values: Record<ArtifactSlotType, number[]>
}

export type HeroAssetArtifactsModel = {
    id: number,
    generation_type: AssetGenerationType,
    base_powers: DifficultyMappedValue,
    powers_grow: DifficultyMappedValue | null,
    required: RequiredArtifacts,
    optional: OptionalArtifacts
}