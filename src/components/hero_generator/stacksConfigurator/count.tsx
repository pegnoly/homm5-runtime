import DifficultyValues from "../artsConfigurator/difficultyValues";
import { AssetGenerationType, DifficultyMappedValue } from "../artsConfigurator/types";
import { HeroAssetStackModel, StackCountGenerationType } from "./main";

function HeroAssetStackCountConfigurator(params: {
    model: HeroAssetStackModel
    updateModelCallback: (value: HeroAssetStackModel) => void
}) {

    async function updateBasePowers(value: DifficultyMappedValue) {
        params.updateModelCallback({...params.model, base_powers: value});
    }

    async function updatePowersGrow(value: DifficultyMappedValue) {
        params.updateModelCallback({...params.model, powers_grow: value});
    }

    async function updateConcreteCounts(value: DifficultyMappedValue) {
        params.updateModelCallback({...params.model, concrete_count: value});
    }

    return <div style={{display: 'flex', width: '95%', borderBottom: 'solid'}}>
    {
        params.model.count_generation_mode == StackCountGenerationType.PowerBased ?
        <PowerBasedSelector
            stackId={params.model.id}
            basePowers={params.model.base_powers}
            powerBasedGenerationType={params.model.power_based_generation_type}
            powersGrow={params.model.powers_grow!}
            basePowersUpdateCallback={updateBasePowers}
            powersGrowUpdateCallback={updatePowersGrow}
        /> :
        <ConcreteCountSelector
            stackId={params.model.id}
            counts={params.model.concrete_count}
            countsUpdateCallback={updateConcreteCounts}
        />
    }
    </div>
}

function ConcreteCountSelector(params: {
    stackId: number,
    counts: DifficultyMappedValue,
    countsUpdateCallback: (value: DifficultyMappedValue) => void
}) {
    return <div style={{display: 'flex', justifyContent: 'space-around'}}>
        <DifficultyValues
            name="Stack concrete counts"
            tauriFunction="update_stack_concrete_count"
            containerId={params.stackId}
            updateCallback={params.countsUpdateCallback}
            values={params.counts}
        />
    </div>
}

function PowerBasedSelector(params: {
    stackId: number,
    basePowers: DifficultyMappedValue,
    powerBasedGenerationType: AssetGenerationType,
    powersGrow: DifficultyMappedValue,
    basePowersUpdateCallback: (value: DifficultyMappedValue) => void,
    powersGrowUpdateCallback: (value: DifficultyMappedValue) => void
}) {

    async function updateStackBasePowers(value: DifficultyMappedValue) {
        params.basePowersUpdateCallback(value);
    }

    async function updateStackPowersGrow(value: DifficultyMappedValue) {
        params.powersGrowUpdateCallback(value);
    }

    return <div style={{display: 'flex', width: '70%', flexDirection: 'row', justifyContent: 'space-between'}}>
        <DifficultyValues
            name="Stack base powers"
            tauriFunction="update_stack_base_powers"
            values={params.basePowers}
            updateCallback={updateStackBasePowers}
            containerId={params.stackId}
        />
        {
            params.powerBasedGenerationType == AssetGenerationType.Dynamic ?
            <DifficultyValues
                name="Stack powers grow"
                tauriFunction="update_stack_powers_grow"
                values={params.powersGrow}
                updateCallback={updateStackPowersGrow}
                containerId={params.stackId}
            /> :
            null
        }
    </div>
}

export default HeroAssetStackCountConfigurator;