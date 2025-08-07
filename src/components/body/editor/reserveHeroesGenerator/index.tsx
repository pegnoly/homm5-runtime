import styles from '../../styles.module.css';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useQuery } from '@tanstack/react-query';
import { ReservedHero, SkillData } from './types';
import { useCurrentMapId } from '@/stores/common';
import { ReserveHeroesGenerator } from './store';
import ReserveHeroesList from './list';
import { Route, Routes, useNavigate } from 'react-router';
import FocusedReservedHero from './focused';
import ReserveHeroesManager from './manager';
import ReserveHeroesPlayerSelector from './playerSelector';

function ReserveHeroesGeneratorLayout() {
    const navigate = useNavigate();
    const actions = ReserveHeroesGenerator.useActions();
    const [currentPlayer, setCurrentPlayer] = useState<number>(1)
    const [heroes, setHeroes] = useState<ReservedHero[]>([]);

    async function playerSelected(value: number) {
        setCurrentPlayer(value);
        actions.unloadReserveHero();
        navigate({pathname: "/editor/ReserveHeroes"})
    }

    async function heroCreated(value: ReservedHero) {
        setHeroes([...heroes, value]);
    }

    async function heroDeleted(value: number) {
        setHeroes(heroes.filter(h => h.id !== value));
    }

    return (
        <>
            <div className={styles.editor_layout}>
                <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column'}}>
                    <div style={{width: '100%', height: '10%', display: 'flex', flexDirection: 'row', alignItems: 'center', justifyContent: 'space-around'}}>
                        <ReserveHeroesPlayerSelector 
                            current={currentPlayer} 
                            onSelected={playerSelected}
                        />
                        <ReserveHeroesManager 
                            player={currentPlayer} 
                            onCreate={heroCreated} 
                            onDelete={heroDeleted}
                        />
                    </div>
                    <div style={{width: '100%', height: '90%'}}>
                        <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
                            <ReserveHeroesList heroes={heroes}/>
                            <Routes>
                                <Route path='/'/>
                                <Route path='/focused/:id' element={<FocusedReservedHero/>}/>
                            </Routes>
                        </div>
                    </div>
                </div>
            </div>
            <ReservedHeroesLoader player={currentPlayer} onLoad={setHeroes}/>
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

function ReservedHeroesLoader({player, onLoad}: { player: number, onLoad: (values: ReservedHero[]) => void }) {
    const mapId = useCurrentMapId();    
    const { data } = useCurrentReservedHeroes(mapId!, player);

    useEffect(() => {
        if (data != undefined) {
            onLoad(data)
        }
    }, [data])

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

    useEffect(() => {
        if (data != undefined) {
            actions.loadBaseSkills(data);
        }
    }, [data])

    return null;
}

export default ReserveHeroesGeneratorLayout;