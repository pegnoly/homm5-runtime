import styles from '../styles.module.css';
import FightAssetStackCountsData from './countsData';
import { useCurrentStackActions } from './store';
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
    const currentStackActions = useCurrentStackActions();

    console.log("StackId: ", stackId);

    const { data } = useStackQuery(stackId);

    if (data != undefined) {
        currentStackActions.loadAsset(data!)
    }

    return null;
}

export default FightAssetFocusedStack;