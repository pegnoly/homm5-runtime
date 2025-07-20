import { invoke } from "@tauri-apps/api/core";
import { ReserveHeroesGenerator } from "../../store";
import { ConcreteSkill, Mastery } from "../../types";
import { Select } from "@mantine/core";
import PerksSelector from "./perks";

function ReservedHeroSkillItem({skill}: {skill: ConcreteSkill}) {

    const currentId = ReserveHeroesGenerator.useCurrentId();
    const actions = ReserveHeroesGenerator.useActions();
    const skills = ReserveHeroesGenerator.useSkills();
    const baseSkills = ReserveHeroesGenerator.useBaseSkills();

    async function updateSkill(value: string) {
        await invoke("update_skill", {id: currentId, slot: skill.slot, skill: {...skill, skill: value}});
        const skillsToUpdate = skills?.skills;
        const updatedSkills = skillsToUpdate?.map(s => {
            if (s.slot == skill.slot) {
                s.skill = value;
                return s;
            } 
            return s;
        });
        actions.updateSkills({...skills, skills: updatedSkills!});
    }

    async function updateMastery(value: Mastery) {
        await invoke("update_skill", {id: currentId, slot: skill.slot, skill: {...skill, mastery: value}});
        const skillsToUpdate = skills?.skills;
        const updatedSkills = skillsToUpdate?.map(s => {
            if (s.slot == skill.slot) {
                s.mastery = value;
                return s;
            } 
            return s;
        });
        actions.updateSkills({...skills, skills: updatedSkills!});
    }

    return (
    <div style={{display: 'flex', flexDirection: 'column'}}>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', gap: '5%', alignItems: 'center'}}>
            <Select
                radius={0}
                size="xs"
                label="Select skill"
                value={skill.skill}
                onChange={(value) => updateSkill(value!)}
                searchable
                data={baseSkills?.map(skill => ({
                    value: skill.game_id, label: skill.names.names[0]
                }))}
            />
            <MasterySelector current={skill.mastery} onSelected={updateMastery}/>
        </div>
        {
            skill.skill == "HERO_SKILL_NONE" ? null : <PerksSelector skill={skill}/>
        }
    </div>
    )
}

function MasterySelector({current, onSelected}: {current: Mastery, onSelected: (value: Mastery) => void}) {

    return (
    <Select
        radius={0}
        size="xs"
        label="Select mastery"
        value={current}
        onChange={(value) => onSelected(value as Mastery)}
        data={[
            {value: Mastery.None, label: "None"},
            {value: Mastery.Basic, label: "Basic"},
            {value: Mastery.Advanced, label: "Advanced"},
            {value: Mastery.Expert, label: "Expert"},
            {value: Mastery.ExtraExpert, label: "ExtraExpert"},
        ]}
    />
    )
}

export default ReservedHeroSkillItem;