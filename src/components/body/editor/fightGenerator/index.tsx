import styles from '../../styles.module.css';
import FightAssetCreator from './elements/assetCreator';
import { useEffect, useState } from 'react';
import { FightAssetSimple } from './types';
import { Route, Routes } from 'react-router';
import FightAssetsList from './elements/assetsList';
import FightAssetFocused from './elements/assetFocused';
import { invoke } from '@tauri-apps/api/core';

function FightGeneratorLayout() {
    const [assets, setAssets] = useState<FightAssetSimple[]>([]);

    useEffect(() => {
        loadAssets();
    }, [])

    const loadAssets = async () => {
        await invoke<FightAssetSimple[]>("load_all_assets")
            .then((data) => setAssets(data));
    }

    async function assetCreated(value: FightAssetSimple) {
        setAssets([...assets, value]);
    }

    return (
    <Routes>
        <Route 
            path='/*' 
            element={
                <div className={styles.editor_layout}>
                    <FightAssetCreator assetCreatedCallback={assetCreated}/>
                    <FightAssetsList assets={assets}/>
                </div>
            }
        />
        <Route path='/focused/:id' element={<FightAssetFocused/>}/>
    </Routes>
    )
}

export default FightGeneratorLayout;