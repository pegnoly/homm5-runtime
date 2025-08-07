import { invoke } from "@tauri-apps/api/core";
import { ReserveHeroesGenerator } from "../../store";
import ReservedHeroSkillItem from "./item";
import { ConcreteSkill } from "../../types";
import { ActionIcon, Button, Card, CardSection, SimpleGrid } from "@mantine/core";
import { IconX } from "@tabler/icons-react";

function ReserveHeroSkillsPanel() {
    const id = ReserveHeroesGenerator.useCurrentId();
    const freeSlots = ReserveHeroesGenerator.useFreeSlots();
    const skills = ReserveHeroesGenerator.useSkills();
    const actions = ReserveHeroesGenerator.useActions();

    async function addSkill() {
        const slot = freeSlots![0];
        await invoke<ConcreteSkill>("add_skill", {id: id, slot: slot})
            .then((value) => {
                const skillsToUpdate = skills?.skills;
                const updatedSkills = [...skillsToUpdate!, value];
                actions.updateSkills({...skills, skills: updatedSkills!});
                const updatedFreeSlots = freeSlots?.filter(s => s != slot);
                actions.updateFreeSlots(updatedFreeSlots!);
            })
    }

    async function removeSkill(slot: number) {
        await invoke("remove_skill", {id: id, slot: slot})
            .then(() => {
                const skillsToUpdate = skills?.skills;
                const updatedSkills = skillsToUpdate?.filter(s => s.slot != slot);
                actions.updateSkills({...skills, skills: updatedSkills!});
                const updatedFreeSlots = [...freeSlots!, slot];
                actions.updateFreeSlots(updatedFreeSlots);
            })
    }
    
    return (
        <>
        {id == undefined ? null : 
            <>
                <Button 
                    onClick={addSkill} 
                    disabled={skills?.skills.length == 6} 
                    radius={0}
                >Add skill</Button>
                <SimpleGrid cols={{ lg: 2, sm: 2 }}>{skills?.skills.map((s, i) => (
                    <Card key={i} radius={0} withBorder>
                        <CardSection>
                            <ActionIcon 
                                radius={0} 
                                size="xs" 
                                bg="red" 
                                style={{ display: 'flex', justifySelf: 'end'}}
                                disabled={s.slot == 0}
                                onClick={() => removeSkill(s.slot)}
                            >
                                <IconX/>
                            </ActionIcon>
                        </CardSection>
                        <ReservedHeroSkillItem skill={s}/>            
                    </Card>
                ))}</SimpleGrid>
            </>}
        </>
    )
}

export default ReserveHeroSkillsPanel;