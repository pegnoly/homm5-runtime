import { useEffect, useState } from 'react';
import styles from '../styles.module.css';
import FightAssetStackSelector from './stackSelector';
import { invoke } from '@tauri-apps/api/core';
import FightAssetStackCreator from './stackCreator';
import FightAssetFocusedStack from './focusedStack';
import { UUID } from 'crypto';
import { ActionIcon, Tooltip } from '@mantine/core';
import { IconX } from '@tabler/icons-react';
import { useMutation } from '@tanstack/react-query';
import { FightGeneratorApi } from '../../api';

function FightAssetStackGenerator({assetId}: {assetId: UUID}) {
    const [stacksIds, setStacksIds] = useState<number []>([]);
    const [selectedStack, setSelectedStack] = useState<number | undefined>(undefined);

    useEffect(() => {
        loadStacksIds();
    }, []);

    const loadStacksIds = async () => {
        await invoke<number[]>("load_stacks_ids", {assetId: assetId})
            .then((data) => setStacksIds(data));
    }

    async function onStackCreated(value: number) {
        setStacksIds([...stacksIds, value]);
    }

    const deleteStackMutation = useMutation({
        mutationFn: async(stackId: number) => {
            return FightGeneratorApi.deleteStack(stackId);
        },
        onSuccess: () => {
            setStacksIds(stacksIds.filter(id => id !== selectedStack));
            setSelectedStack(undefined);
        }
    });

    return (
    <div className={styles.stacks_panel}>
        <div className={styles.stack_selector_panel}>
            <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '2%'}}>
                <div style={{display: 'flex', flexDirection: 'row', gap: 10, alignItems: 'center'}}>
                    <FightAssetStackCreator
                        assetId={assetId}
                        disabled={stacksIds.length >= 7}
                        stackCreatedCallback={onStackCreated}
                    />
                    <Tooltip label="Use to delete selected stack">
                        <ActionIcon bg="red" onClick={() => deleteStackMutation.mutate(selectedStack!)} disabled={selectedStack == undefined}>
                            <IconX/>
                        </ActionIcon>
                    </Tooltip>
                </div>
                <FightAssetStackSelector 
                    stackIds={stacksIds}
                    currentSelectedStack={selectedStack}
                    stackSelectedCallback={setSelectedStack}
                />
            </div>
        </div>
        <FightAssetFocusedStack stackId={selectedStack!}/>
    </div>
    )
}

export default FightAssetStackGenerator;