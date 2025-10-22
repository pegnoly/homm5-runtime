import styles from '../../styles.module.css';
import FightAssetCreator from './elements/assetCreator';
import { useEffect, useState } from 'react';
import { FightAssetSimple } from './types';
import { Route, Routes } from 'react-router';
import FightAssetsList from './elements/assetsList';
import FightAssetFocused from './elements/assetFocused';
import { invoke } from '@tauri-apps/api/core';
import { useCurrentMapId } from '@/stores/common';
import { useQuery } from '@tanstack/react-query';
import { UUID } from 'crypto';

function FightGeneratorLayout() {
    const [assets, setAssets] = useState<FightAssetSimple[]>([]);

    async function assetCreated(value: FightAssetSimple) {
        setAssets([...assets, value]);
    }

    async function assetDeleted(value: UUID) {
        setAssets(assets.filter(a => a.id != value));
    }

    async function assetSheetCreated(id: UUID, sheetId: number) {
        setAssets(assets.map(a => {
            if (a.id == id) {
                a.sheet_id = sheetId
            }
            return a;
        }))
    }

    return (
    <>
        <Routes>
            <Route 
                path='/*' 
                element={
                    <div className={styles.editor_layout}>
                        <FightAssetCreator onCreated={assetCreated}/>
                        <FightAssetsList assets={assets} onAssetDeleted={assetDeleted} onAssetSheetCreated={assetSheetCreated}/>
                    </div>
                }
            />
            <Route path='/focused/:id' element={<FightAssetFocused/>}/>
        </Routes>
        <FightAssetsLoader onLoad={setAssets}/>
    </>
    )
}

function useFightAssets(mapId: number) {
    return useQuery({
        queryKey: ['fight_assets', mapId],
        queryFn: async() => {
           return invoke<FightAssetSimple[]>("load_all_assets", {mapId: mapId}); 
        }
    })
}

function FightAssetsLoader({onLoad}: {onLoad: (values: FightAssetSimple[]) => void}) {
    const currentMapId = useCurrentMapId();
    const { data } = useFightAssets(currentMapId!);
    useEffect(() => {
        if (data != undefined) {
            onLoad(data);
        }
    }, [data]);

    return null;
}

export default FightGeneratorLayout;