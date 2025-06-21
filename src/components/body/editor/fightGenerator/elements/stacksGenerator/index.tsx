import { useEffect, useState } from 'react';
import styles from '../styles.module.css';
import FightAssetStackSelector from './stackSelector';
import { invoke } from '@tauri-apps/api/core';
import FightAssetStackCreator from './stackCreator';
import FightAssetFocusedStack from './focusedStack';

function FightAssetStackGenerator(params: {
    assetId: number
}) {
    const [stacksIds, setStacksIds] = useState<number []>([]);
    const [selectedStack, setSelectedStack] = useState<number | undefined>(undefined);

    useEffect(() => {
        loadStacksIds();
    }, []);

    const loadStacksIds = async () => {
        await invoke<number[]>("load_stacks_ids", {assetId: params.assetId})
            .then((data) => setStacksIds(data));
    }

    async function onStackCreated(value: number) {
        setStacksIds([...stacksIds, value]);
    }

    return (
    <div className={styles.stacks_panel}>
        <div className={styles.stack_selector_panel}>
            <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1%'}}>
                <FightAssetStackSelector 
                    stackIds={stacksIds}
                    currentSelectedStack={selectedStack}
                    stackSelectedCallback={setSelectedStack}
                />
                <FightAssetStackCreator
                    assetId={params.assetId}
                    disabled={stacksIds.length >= 7}
                    stackCreatedCallback={onStackCreated}
                />
            </div>
        </div>
        <FightAssetFocusedStack assetId={params.assetId} stackId={selectedStack!}/>
    </div>
    )
}

export default FightAssetStackGenerator;