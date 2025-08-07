import { useEffect, useState } from "react";
import { ReserveHeroesGenerator } from "../../store";
import { ConcreteSkill, PerkData } from "../../types";
import { invoke } from "@tauri-apps/api/core";
import { MultiSelect } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";

function PerksSelector({skill}: { skill: ConcreteSkill }) {

    const currentId = ReserveHeroesGenerator.useCurrentId();
    const skills = ReserveHeroesGenerator.useSkills();
    const actions = ReserveHeroesGenerator.useActions();

    const [perks, setPerks] = useState<PerkData[]>([]);

    async function updatePerks(values: string[]) {
        await invoke("update_skill", {id: currentId, slot: skill.slot, skill: {...skill, perks: values}});
        const skillsToUpdate = skills?.skills;
        const updatedSkills = skillsToUpdate?.map(s => {
            if (s.slot == skill.slot) {
                s.perks = values;
                return s;
            } 
            return s;
        });
        actions.updateSkills({...skills, skills: updatedSkills!});
    }

    return (
    <>
        <MultiSelect
            radius={0}
            size="xs"
            label="Select perks"
            maxValues={skill.slot == 0 ? 4: 3}
            value={skill.perks}
            onChange={(values) => updatePerks(values)}
            data={perks.map(p => ({
                value: p.game_id, label: p.names.names[0]
            }))}
        />
        <PerksLoader skill={skill.skill} onLoad={setPerks}/>
    </>
    )
}

function usePerks(skill: string) {
    return useQuery({
        queryKey: ["perks", skill],
        queryFn: async() => {
            return invoke<PerkData[]>("load_perks", {skill: skill});
        }
    })
}

function PerksLoader({skill, onLoad}: {skill: string, onLoad: (values: PerkData[]) => void}) {
    const { data } = usePerks(skill);

    useEffect(() => {
        if (data != undefined) {
            onLoad(data);
        }
    }, [data]);
    
    return null;
}

export default PerksSelector;