import styles from '../styles.module.css';
import FightAssetStackCountsData from './countsData';
import { useCurrentStackActions, useCurrentStackId } from './store';
import FightAssetStackParamsData from './generationParams';
import { useQuery } from '@tanstack/react-query';
import { FightGeneratorApi } from '../../api';

function useStackQuery(stackId: number) {
    return useQuery({
        queryKey: ["fight_asset_current_stack", stackId],
        queryFn: async() => {
            return FightGeneratorApi.loadStack(stackId);
        }
    })
}

function FightAssetFocusedStack(params: {
    assedId: number,
    stackId: number
}) {
    const currentStackId = useCurrentStackId();
    const currentStackActions = useCurrentStackActions();

    const { data } = useStackQuery(params.stackId);

    if (data != undefined) {
        currentStackActions.loadAsset(data!)
    }

    return (
    <div className={styles.focused_stack_panel}>
        {
            currentStackId != undefined ?
            <>
                <FightAssetStackCountsData/>
                <FightAssetStackParamsData/>
            </> :
            null
        }
    </div> 
    )
}

export default FightAssetFocusedStack;