import { useParams } from "react-router";
import ReserveHeroSpellsPanel from "./spells/panel";
import ReserveHeroSkillsPanel from "./skills/panel";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { ReservedHeroFull } from "../types";
import { ReserveHeroesGenerator } from "../store";
import FavoriteEnemiesBase from "./favoriteEnemies";

function FocusedReservedHero() {
    const { id } = useParams();

    return (
    <>
        {
            id == undefined ? null :
            <>
                <div style={{width: '90%', height: '100%'}}>
                    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row', gap: '2%'}}>
                        <div style={{width: '69%', height: '100%'}}>
                            <ReserveHeroSkillsPanel/>
                        </div>
                        <div style={{width: '29%', height: '100%'}}>
                            <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column'}}>
                                <div style={{width: '100%', height: '60%'}}>
                                    <ReserveHeroSpellsPanel/>
                                </div>
                                <div style={{width: '100%', height: '40%'}}>
                                    <FavoriteEnemiesBase/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <FocusedHeroLoader id={parseInt(id!)}/>
            </>
        }
    </>

    )
}

function useReservedHero(id: number) {
    return useQuery({
        queryKey: ['reserved_hero', id],
        queryFn: async() => {
            return invoke<ReservedHeroFull>("load_existing_reserved_hero", {id: id});
        } 
    })
}

function FocusedHeroLoader({id}: {id: number}) {
    const actions = ReserveHeroesGenerator.useActions();
    const { data } = useReservedHero(id);

    if (data != undefined) {
        actions.loadReservedHero(data);
    }

    return null;
}

export default FocusedReservedHero;