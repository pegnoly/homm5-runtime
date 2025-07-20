import styles from '../../styles.module.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useQuery } from '@tanstack/react-query';
import { ReservedHero, SkillData } from './types';
import { useCurrentMapId } from '@/stores/common';
import { ReserveHeroesGenerator } from './store';
import ReserveHeroesGeneratorPlayerSelector from './playerSelector';
import ReserveHeroesGeneratorHeroCreator from './heroCreator';
import ReserveHeroesList from './list';
import { Route, Routes } from 'react-router';
import FocusedReservedHero from './focused';

function ReserveHeroesGeneratorLayout() {
    const [currentPlayer, setCurrentPlayer] = useState<number>(1)

    return (
        <>
            <div className={styles.editor_layout}>
                <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column'}}>
                    <div style={{width: '100%', height: '10%', display: 'flex', flexDirection: 'row', alignItems: 'center', justifyContent: 'space-around'}}>
                        <ReserveHeroesGeneratorPlayerSelector current={currentPlayer} onSelected={setCurrentPlayer}/>
                        <ReserveHeroesGeneratorHeroCreator player={currentPlayer}/>
                    </div>
                    <div style={{width: '100%', height: '90%'}}>
                        <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
                            <ReserveHeroesList/>
                            <Routes>
                                <Route path='/'/>
                                <Route path='/focused/:id' element={<FocusedReservedHero/>}/>
                            </Routes>
                        </div>
                    </div>
                </div>
            </div>
            <ReservedHeroesLoader player={currentPlayer}/>
            <BaseSkillsLoader/>
        </>
    )
}

function useCurrentReservedHeroes(mapId: number, player: number) {
    return useQuery({
        queryFn: async() => {
            return invoke<ReservedHero[]>("load_heroes", {mapId: mapId, player: player});
        },
        queryKey: ["reserved_heroes", mapId, player]
    });
}

function ReservedHeroesLoader({player}: { player: number }) {
    const mapId = useCurrentMapId();
    const actions = ReserveHeroesGenerator.useActions();
    
    const { data } = useCurrentReservedHeroes(mapId!, player);

    if (data != undefined) {
        actions.loadHeroes(data);
    } else {
        return null
    }

    return null;
}

function useBaseSkills() {
    return useQuery({
        queryKey: ["base_skills"],
        queryFn: async() => {
            return invoke<SkillData[]>("load_base_skills");
        }
    })
}

function BaseSkillsLoader() {
    const actions = ReserveHeroesGenerator.useActions();

    const { data } = useBaseSkills();
    if (data != undefined) {
        actions.loadBaseSkills(data);
    }

    return null;
}

export default ReserveHeroesGeneratorLayout;