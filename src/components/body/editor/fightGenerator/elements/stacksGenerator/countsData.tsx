import { AssetGenerationType } from '../../types';
import DifficultyValues from '../common/difficultyValues';
import styles from '../styles.module.css';
import FightAssetCurrentStackData from './data';
import {
    FightStackGeneratorStore
} from './store';
import AverageTownsTiersTooltip from './tooltips/towsTiersTooltip';
import { CalculationType, SelectionType } from './types';
import SpecificCreaturesTooltip from "./tooltips/concreteCreaturesTooltip";

function FightAssetStackCountsData() {
    const calculationType = FightStackGeneratorStore.useCalculationType();
    return (
    <div className={styles.stack_counts_panel}>
        <div style={{display: 'flex', flexDirection: 'row', gap: '5%'}}>
            <FightAssetCurrentStackData/>
        </div>
        {
            calculationType == CalculationType.PowerBased ?
            <PowerBasedSelector/> :
            <SpecificCountSelector/>
        }
    </div> 
    )
}

function SpecificCountSelector() {
    const id = FightStackGeneratorStore.useCurrentId();
    const specificCounts = FightStackGeneratorStore.useSpecificCounts();
    const actions = FightStackGeneratorStore.useActions();

    return <div style={{display: 'flex', justifyContent: 'space-around'}}>
        {
            id == undefined ? null :
            <DifficultyValues
                name="Конкретные числа юнитов"
                tauriFunction="update_stack_specific_count"
                containerId={id!}
                updateCallback={actions.setSpecificCounts}
                values={specificCounts!}
            />
        }
    </div>
}

function PowerBasedSelector() {
    const id = FightStackGeneratorStore.useCurrentId();
    const updateType = FightStackGeneratorStore.useUpdateType();
    const basePowers = FightStackGeneratorStore.useBasePowers();
    const powersGrow = FightStackGeneratorStore.usePowersGrow();
    const actions = FightStackGeneratorStore.useActions();
    const selectionType = FightStackGeneratorStore.useSelectionType()

    return <div style={{display: 'flex', width: '100%', flexDirection: 'row', justifyContent: 'space-between'}}>
    {
        id == undefined ? null :
        (
            selectionType == SelectionType.TierSlotBased ?
            <>
                <div style={{width: '50%', display: 'flex'}}>
                    <DifficultyValues
                        name="Базовая сила отряда"
                        tauriFunction="update_stack_base_powers"
                        values={basePowers!}
                        updateCallback={actions.setBasePowers}
                        containerId={id!}
                        tooltipComponent={{component: AverageTownsTiersTooltip}}
                    />
                </div>
                {
                    updateType == AssetGenerationType.Dynamic ?
                    <div style={{width: '50%', display: 'flex'}}>
                        <DifficultyValues
                            name="Прирост силы отряда"
                            tauriFunction="update_stack_powers_grow"
                            values={powersGrow!}
                            updateCallback={actions.setPowersGrow}
                            containerId={id!}
                            tooltipComponent={{component: AverageTownsTiersTooltip}}
                        />
                    </div> :
                    null
                }
            </> :
            <>
                <div style={{width: '50%', display: 'flex'}}>
                    <DifficultyValues
                        name="Базовая численность отряда"
                        tauriFunction="update_stack_specific_count"
                        values={basePowers!}
                        updateCallback={actions.setSpecificCounts}
                        containerId={id!}
                        tooltipComponent={{component: SpecificCreaturesTooltip}}
                    />
                </div>
                {
                    updateType == AssetGenerationType.Dynamic ?
                    <div style={{width: '50%', display: 'flex'}}>
                        <DifficultyValues
                            name="Прирост численности отряда"
                            tauriFunction="update_stack_powers_grow"
                            values={powersGrow!}
                            updateCallback={actions.setPowersGrow}
                            containerId={id!}
                            tooltipComponent={{component: SpecificCreaturesTooltip}}
                        />
                    </div> :
                    null
                }
            </>
        )
    }
    </div>
}

export default FightAssetStackCountsData;