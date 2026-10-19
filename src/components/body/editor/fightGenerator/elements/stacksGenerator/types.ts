import {AssetGenerationType, DifficultyMappedValue, TownTypeExtended} from "../../types"

export enum SelectionType {
    Manual = "MANUAL",
    TierSlotBased = "TIER_SLOT_BASED"
}

export const unitSelectionTypeNames = new Map<SelectionType, string>([
    [SelectionType.Manual, "Manual"],
    [SelectionType.TierSlotBased, "Tier-slot based"]
])

export enum CalculationType {
    Raw = "RAW",
    PowerBased = "POWER_BASED"
}

export const unicCalculationTypeNames = new Map<CalculationType, string>([
    [CalculationType.Raw, "Raw"],
    [CalculationType.PowerBased, "Power based"]
])

export enum StackGenerationParam {
    UpgradeOnly = "UPGRADE_ONLY",
    Generatable = "GENERATABLE",
    Shooter = "SHOOTER",
    Caster = "CASTER"
}

export const generationParamsNames = new Map<StackGenerationParam, string>([
    [StackGenerationParam.UpgradeOnly, "Upgrade only"],
    [StackGenerationParam.Generatable, "Generatable only"],
    [StackGenerationParam.Caster, "Casters only"],
    [StackGenerationParam.Shooter, "Shooters only"]
])

export type StackGenerationRules = {
    rules: StackGenerationParam[]
}

export enum StatGenerationType {
    Initiative = "INITIATIVE",
    Speed = "SPEED",
    Hitpoints = "HITPOINTS",
    Attack = "ATTACK",
    Defence = "DEFENCE"
}

export enum StatGenerationRule {
    MaxBy = "MAXBY",
    MinBy = "MINBY"
}

export type StatRuleModel = {
    param: StatGenerationType,
    rule: StatGenerationRule
}
export type StackStatsRules = {
    rules: StatRuleModel[]
}

export type CreatureTowns = {
    towns: TownTypeExtended[]
}

export type CreatureTiers = {
    tiers: number []
}

export type CreatureIds = {
    ids: number []
}

export type FightAssetStackModel = {
    id: number,
    selection_type: SelectionType,
    calculation_type: CalculationType,
    update_type: AssetGenerationType,
    base_powers: DifficultyMappedValue,
    powers_grow: DifficultyMappedValue,
    towns: CreatureTowns,
    tiers: CreatureTiers,
    rules: StackGenerationRules,
    specific_creatures: CreatureIds,
    specific_count: DifficultyMappedValue,
    stat_rules: StackStatsRules
}