import { UUID } from "crypto";

export type FightAssetSimple = {
    id: UUID,
    name: string,
    sheet_id: number | null
}

export enum AssetGenerationType {
    Static = "GENERATION_TYPE_STATIC",
    Dynamic = "GENERATION_TYPE_DYNAMIC"
}

export enum DifficultyType {
    Easy = "DIFFICULTY_EASY",
    Medium = "DIFFICULTY_NORMAL",
    Hard = "DIFFICULTY_HARD",
    Heroic = "DIFFICULTY_HEROIC"
}

export enum TownType {
    TownNoType = "TOWN_NO_TYPE",
    TownHeaven = "TOWN_HEAVEN",
    TownInferno = "TOWN_INFERNO",
    TownNecromancy = "TOWN_NECROMANCY", 
    TownPreserve = "TOWN_PRESERVE",
    TownDungeon = "TOWN_DUNGEON",
    TownAcademy = "TOWN_ACADEMY",
    TownFortress = "TOWN_FORTRESS",
    TownStronghold = "TOWN_STRONGHOLD"
}

export const townsNames = new Map<TownType, string>([
    [TownType.TownNoType, "Neutral"],
    [TownType.TownHeaven, "Heaven"],
    [TownType.TownInferno, "Inferno"],
    [TownType.TownNecromancy, "Necromancy"],
    [TownType.TownPreserve, "Preserve"],
    [TownType.TownDungeon, "Dungeon"],
    [TownType.TownAcademy, "Academy"],
    [TownType.TownFortress, "Fortress"],
    [TownType.TownStronghold, "Stronghold"]
]);

export type DifficultyMappedValue = {
    data: Record<DifficultyType, number>
}

export const gameDifficultyNames = new Map<DifficultyType, string>([
    [DifficultyType.Easy, "Easy"],
    [DifficultyType.Medium, "Medium"],
    [DifficultyType.Hard, "Hard"],
    [DifficultyType.Heroic, "Heroic"]
])