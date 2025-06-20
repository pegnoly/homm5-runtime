import { AssetGenerationType, DifficultyMappedValue, TownType } from "../../types"

export enum StackUnitGenerationType {
    ConcreteUnit = "UNIT_TYPE_GENERATION_MODE_CONCRETE",
    TierSlotBased = "UNIT_TYPE_GENERATION_MODE_TIER_SLOT_BASED"
}

export const unitGenerationTypeNames = new Map<StackUnitGenerationType, string>([
    [StackUnitGenerationType.ConcreteUnit, "Concrete unit"],
    [StackUnitGenerationType.TierSlotBased, "Tier-slot based"]
])

export enum StackCountGenerationType {
    Raw = "UNIT_COUNT_GENERATION_MODE_RAW",
    PowerBased = "UNIT_COUNT_GENERATION_MODE_POWER_BASED"
}

export const countGenerationTypeNames = new Map<StackCountGenerationType, string>([
    [StackCountGenerationType.Raw, "Concrete count"],
    [StackCountGenerationType.PowerBased, "Power based"]
])

export enum StackGenerationParam {
    UpgradeOnly = "GENERATION_RULE_UPGRADE_ONLY",
    Generatable = "GENERATION_RULE_GENERATABLE",
    Shooter = "GENERATION_RULE_SHOOTER",
    Caster = "GENERATION_RULE_CASTER"
}

export const generationParamsNames = new Map<StackGenerationParam, string>([
    [StackGenerationParam.UpgradeOnly, "Upgrade only"],
    [StackGenerationParam.Generatable, "Generatable only"],
    [StackGenerationParam.Caster, "Casters only"],
    [StackGenerationParam.Shooter, "Shooters only"]
])

export enum StatGenerationType {
    Initiative = "GENERATION_STAT_INITIATIVE",
    Speed = "GENERATION_STAT_SPEED",
    Hitpoints = "GENERATION_STAT_HITPOINTS",
    Attack = "GENERATION_STAT_ATTACK",
    Defence = "GENERATION_STAT_DEFENCE"
}

export enum StatGenerationRule {
    MaxBy = "GENERATION_STAT_RULE_MAXBY",
    MinBy = "GENERATION_STAT_RULE_MINBY"
}

export type ArmyGenerationStats = {
    values: StatGenerationType[]
}

export type ArmyGenerationStatElement = {
    id: number,
    rule: StatGenerationRule,
    priority: number,
    stats: ArmyGenerationStats
}

export type StackGenerationRules = {
    params: StackGenerationParam[]
}

export type CreatureTowns = {
    towns: TownType[]
}

export type CreatureTiers = {
    tiers: number []
}

export type CreatureIds = {
    ids: number []
}

export type FightAssetStackModel = {
    id: number,
    type_generation_mode: StackUnitGenerationType,
    count_generation_mode: StackCountGenerationType,
    power_based_generation_type: AssetGenerationType,
    base_powers: DifficultyMappedValue,
    powers_grow: DifficultyMappedValue,
    towns: CreatureTowns,
    tiers: CreatureTiers,
    generation_rule: StackGenerationRules,
    concrete_creatures: CreatureIds,
    concrete_count: DifficultyMappedValue
}