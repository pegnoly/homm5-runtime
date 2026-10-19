import styles from '../styles.module.css';
import FightAssetStackCountsData from './countsData';
import { FightStackGeneratorStore } from './store';
import FightAssetStackParamsData from './generationParams';
import { useQuery } from '@tanstack/react-query';
import { FightGeneratorApi } from '../../api';

function FightAssetFocusedStack({stackId}: {stackId: number}) {

    return (
    <div className={styles.focused_stack_panel}>
        {
            stackId != undefined ?
            <>
                <FightAssetStackCountsData/>
                <FightAssetStackParamsData/>
                <StackLoader stackId={stackId}/>
            </> :
            null
        }
    </div> 
    )
}

function useStackQuery(stackId: number) {
    return useQuery({
        queryKey: ["fight_asset_current_stack", stackId],
        queryFn: async() => {
            return FightGeneratorApi.loadStack(stackId);
        }
    })
}

function StackLoader({stackId}: {stackId: number}) {
    console.log("Loading stack with id ", stackId)
    const actions = FightStackGeneratorStore.useActions();
    const { data } = useStackQuery(stackId);
    console.log("Data ",  data)
    if (data != undefined) {
        actions.loadAsset(data!)
    }

    return null;
}

export default FightAssetFocusedStack;