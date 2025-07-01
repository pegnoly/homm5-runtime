import { AssetGenerationType } from '../../types';
import DifficultyValues from '../common/difficultyValues';
import styles from '../styles.module.css';
import FightAssetCurrentStackData from './data';
import AverageCreaturesTooltip from './helper';
import { useBasePowers, useConcreteCounts, useCountGenerationMode, useCurrentStackActions, useCurrentStackId, usePowerBasetGenerationType, usePowersGrow } from './store';
import { StackCountGenerationType } from './types';

function FightAssetStackCountsData() {
    const countGenerationType = useCountGenerationMode();
    return (
    <div className={styles.stack_counts_panel}>
        <div style={{display: 'flex', flexDirection: 'row', gap: '5%'}}>
            <FightAssetCurrentStackData/>
        </div>
        {
            countGenerationType == StackCountGenerationType.PowerBased ?
            <PowerBasedSelector/> :
            <ConcreteCountSelector/>
        }
    </div> 
    )
}

export default FightAssetStackCountsData;

function ConcreteCountSelector() {
    const currentStackId = useCurrentStackId();
    const concreteCounts = useConcreteCounts();
    const actions = useCurrentStackActions();

    return <div style={{display: 'flex', justifyContent: 'space-around'}}>
        <DifficultyValues
            name="Stack concrete counts"
            tauriFunction="update_stack_concrete_count"
            containerId={currentStackId!}
            updateCallback={actions.setConcreteCounts}
            values={concreteCounts!}
        />
    </div>
}

function PowerBasedSelector() {
    const currentStackId = useCurrentStackId();
    const generationType = usePowerBasetGenerationType();
    const basePowers = useBasePowers();
    const powersGrow = usePowersGrow();
    const actions = useCurrentStackActions();

    return <div style={{display: 'flex', width: '100%', flexDirection: 'row', justifyContent: 'space-between'}}>
        <div style={{width: '50%', display: 'flex'}}>
            <DifficultyValues
                name="Stack base powers"
                tauriFunction="update_stack_base_powers"
                values={basePowers!}
                updateCallback={actions.setBasePowers}
                containerId={currentStackId!}
                tooltipComponent={{component: AverageCreaturesTooltip}}
            />
        </div>
        {
            generationType == AssetGenerationType.Dynamic ?
            <div style={{width: '50%', display: 'flex'}}>
                <DifficultyValues
                    name="Stack powers grow"
                    tauriFunction="update_stack_powers_grow"
                    values={powersGrow!}
                    updateCallback={actions.setPowersGrow}
                    containerId={currentStackId!}
                    tooltipComponent={{component: AverageCreaturesTooltip}}
                />
            </div> :
            null
        }
    </div>
}