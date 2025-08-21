import { Checkbox, MultiSelect, Stack } from "@mantine/core";
import { TownType } from "../../../fightGenerator/types";
import { ReserveHeroesGenerator } from "../../store";
import useGameDataStore from "@/stores/GameDataStore";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Mastery } from "../../types";

function FavoriteEnemiesBase() {
    const town = ReserveHeroesGenerator.useCurrentTown();

    return (
    <>
        {
            town != TownType.TownPreserve ? null : <FavoriteEnemiesSelector/>
        }
    </>
    )
    
}

function FavoriteEnemiesSelector() {
    const enemies = ReserveHeroesGenerator.useFavoriteEnemies();
    const id = ReserveHeroesGenerator.useCurrentId();
    const skills = ReserveHeroesGenerator.useSkills();
    const actions = ReserveHeroesGenerator.useActions();

    const creatures = useGameDataStore(state => state.creatures);

    const [pendantUsed, setPendantUsed] = useState<boolean>(false);

    const mutation = useMutation({
        mutationFn: async(data: {values: string []}) => {
            return invoke("update_favorite_enemies", {id: id, enemies: data.values});
        },
        onSuccess(_data, variables, _context) {
            actions.updateEnemies(variables.values);
        },
    });

    const avengerSkill = skills?.skills.find(s => s.skill == "HERO_SKILL_AVENGER");
    let enemiesCount = 0;
    if (avengerSkill != undefined) {
        switch (avengerSkill.mastery) {
            case Mastery.None: 
                enemiesCount = 0;
                break;
            case Mastery.Basic: 
                enemiesCount = 1 + (pendantUsed ? 1 : 0);
                break;
            case Mastery.Advanced:
                enemiesCount = 2 + (pendantUsed ? 1 : 0);
                break;
            case Mastery.Expert:
                enemiesCount = 3 + (pendantUsed ? 1 : 0);
                break;
            case Mastery.ExtraExpert:
                enemiesCount = 4
                break;
            default:
                break;
        }
    }

    return (
    <Stack>
        <MultiSelect
            label="Select favorite enemies"
            size="xs"
            searchable
            disabled={enemiesCount == 0}
            maxValues={enemiesCount}
            value={enemies?.enemies}
            data={creatures.filter(c => c.id > 0 && c.id < 180).map(c => ({
                label: c.name, value: c.game_id
            }))}
            onChange={(values) => mutation.mutate({values: values})}
        />
        <Checkbox size="xs" checked={pendantUsed} onChange={(e) => setPendantUsed(e.currentTarget.checked)} label="Consider pedant usage"/>
    </Stack>
    )
}

export default FavoriteEnemiesBase;