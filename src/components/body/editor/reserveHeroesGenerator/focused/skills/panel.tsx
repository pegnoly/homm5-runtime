import { invoke } from "@tauri-apps/api/core";
import { ReserveHeroesGenerator } from "../../store";
import ReservedHeroSkillItem from "./item";
import { ConcreteSkill } from "../../types";
import { Button, Card, SimpleGrid } from "@mantine/core";

function ReserveHeroSkillsPanel() {
    const id = ReserveHeroesGenerator.useCurrentId();
    const skills = ReserveHeroesGenerator.useSkills();
    const actions = ReserveHeroesGenerator.useActions();

    async function addSkill() {
        const slot = skills?.skills.length! + 1;
        await invoke<ConcreteSkill>("add_skill", {id: id, slot: slot})
            .then((value) => {
                const skillsToUpdate = skills?.skills;
                const updatedSkills = [...skillsToUpdate!, value];
                actions.updateSkills({...skills, skills: updatedSkills!});
            })
    }
    
    return (
    <>
        <Button 
            onClick={addSkill} 
            disabled={skills?.skills.length == 6} 
            radius={0}
        >Add skill</Button>
        <SimpleGrid cols={{ lg: 2, sm: 2 }}>{skills?.skills.map((s, i) => (
            <Card key={i} radius={0} withBorder>
                <ReservedHeroSkillItem skill={s}/>            
            </Card>
        ))}</SimpleGrid>
    </>
    )
}

export default ReserveHeroSkillsPanel;