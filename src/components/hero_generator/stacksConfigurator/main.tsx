import { useEffect, useState } from "react"
import { AssetGenerationType, DifficultyMappedValue } from "../artsConfigurator/types"
import { invoke } from "@tauri-apps/api/core"
import HeroAssetFocusedStack from "./focused"
import HeroAssetStacksConfiguratorHeader from "./header"
import HeroAssetCurrentStackData from "./data"

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

export type StackGenerationRules = {
    params: StackGenerationParam[]
}

export type HeroAssetStackModel = {
    id: number,
    type_generation_mode: StackUnitGenerationType,
    count_generation_mode: StackCountGenerationType,
    power_based_generation_type: AssetGenerationType,
    base_powers: DifficultyMappedValue,
    powers_grow: DifficultyMappedValue | null,
    town: TownType,
    tier: number,
    generation_rule: StackGenerationRules,
    concrete_creature: number,
    concrete_count: DifficultyMappedValue
}

function HeroAssetStacksConfigurator(params: {
    assetId: number
}) {
    const [stacksIds, setStacksIds] = useState<number[]>([]);
    const [selectedStack, setSelectedStack] = useState<number | null>(null);

    useEffect(() => {
        loadStacksIds();
    }, []);

    const loadStacksIds = async () => {
        await invoke<number[]>("load_stacks_ids", {assetId: params.assetId})
            .then((values) => setStacksIds(values));
    }

    async function onStackCreated(value: number) {
        setStacksIds([...stacksIds, value]);
    }

    return <div style={{display: 'flex', flexDirection: 'column'}}>
        <div style={{width: '100%', height: '15%', display: 'flex'}}>
            <div style={{height: '100%', display: 'flex',  flexDirection: 'column'}}>
                <HeroAssetStacksConfiguratorHeader
                    assetId={params.assetId}
                    stacks={stacksIds}
                    currentStack={selectedStack}
                    stackSelectedCallback={setSelectedStack}
                    stackCreatedCallback={onStackCreated}
                />
                <HeroAssetCurrentStackData/>
            </div>
        </div>
        {
            selectedStack != null ?
            <div style={{width: '100%', height: '82%', display: 'flex', flexDirection: 'column', paddingTop: '3%'}}>
                <HeroAssetFocusedStack stackId={selectedStack}/>
            </div> :
            null
        }
    </div>
}

export default HeroAssetStacksConfigurator;