import { ActionIcon, Button, Card, CardSection, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, SimpleGrid, Stack, Text } from "@mantine/core";
import { CreatureEditorStore } from "../store";
import useGameDataStore from "@/stores/GameDataStore";
import { Mastery } from "../../reserveHeroesGenerator/types";
import { IconX } from "@tabler/icons-react";
import { ObjectUtils } from "@/lib/utils";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function CreatureSpellsEditor() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();
    const spells = useGameDataStore(state => state.spells);

    async function removeSpell(id: string) {
        await invoke("remove_creature_spell", {id: currentCreature?.id, value: id})
            .then(() => {
                const updatedSpells = currentCreature?.known_spells.spells.filter(s => s.spell != id);
                const updatedModel = ObjectUtils.updateObjectDynamically(currentCreature!, "known_spells.spells", updatedSpells);
                actions.updateCreature(updatedModel);
            });
    }

    async function updateSpell(prevId: string, newId: string) {
        const mastery = currentCreature?.known_spells.spells.find(sp => sp.spell == prevId)?.mastery;
        await invoke("update_creature_spell", {id: currentCreature?.id, currSpell: prevId, newSpell: { spell: newId, mastery: mastery }})
            .then(() => {
                const updatedSpells = currentCreature?.known_spells.spells.map(sp => {
                    if (sp.spell == prevId) {
                        sp.spell = newId;
                        return sp;
                    } else {
                        return sp;
                    }
                })
                const updatedModel = ObjectUtils.updateObjectDynamically(currentCreature!, "known_spells.spells", updatedSpells);
                actions.updateCreature(updatedModel);
            })
    }

    async function updateMastery(prevId: string, newMastery: Mastery) {
        await invoke("update_creature_spell", {id: currentCreature?.id, currSpell: prevId, newSpell: { spell: prevId, mastery: newMastery }})
            .then(() => {
                const updatedSpells = currentCreature?.known_spells.spells.map(sp => {
                    if (sp.spell == prevId) {
                        sp.mastery = newMastery;
                        return sp;
                    } else {
                        return sp;
                    }
                })
                const updatedModel = ObjectUtils.updateObjectDynamically(currentCreature!, "known_spells.spells", updatedSpells);
                actions.updateCreature(updatedModel);
            })
    }

    return (
    <>
    {
        currentCreature == undefined ? null :
        <div style={{width: '95%', paddingTop: '7%'}}>
            <Group justify="space-between">
                <Text style={{fontSize: 20}}>Spells</Text>
                <SpellCreator/>
            </Group>
            <SimpleGrid cols={{xs: 2, xl: 3}}>{currentCreature.known_spells.spells.map(s => (
                <Card radius={0} withBorder key={s.spell}>
                    <CardSection>
                        <ActionIcon 
                            radius={0} 
                            size="xs" 
                            bg="red" 
                            style={{ display: 'flex', justifySelf: 'end'}}
                            onClick={() => removeSpell(s.spell)}
                        >
                            <IconX/>
                        </ActionIcon>
                    </CardSection>
                    <Select
                        searchable
                        label="Spell"
                        size="xs"
                        radius={0}
                        value={s.spell}
                        data={
                            spells
                                .filter(sp => !currentCreature.known_spells.spells.some(spt => sp.game_id != s.spell && sp.game_id == spt.spell))
                                .map(sp => ({label: sp.name, value: sp.game_id}))
                        }
                        onChange={(value) => updateSpell(s.spell, value!)}
                    />
                    <Select
                        label="Mastery"
                        size="xs"
                        radius={0}
                        value={s.mastery}
                        data={[
                            {label: 'None', value: Mastery.None},
                            {label: 'Basic', value: Mastery.Basic},
                            {label: 'Advanced', value: Mastery.Advanced},
                            {label: 'Expert', value: Mastery.Expert},
                            {label: 'ExtraExpert', value: Mastery.ExtraExpert},
                        ]}
                        onChange={(value) => updateMastery(s.spell, value as Mastery)}
                    />
                </Card>
            ))}</SimpleGrid>
        </div>
    }
    </>
    )
}

function SpellCreator() {
    const [opened, {open, close}] = useDisclosure(false);
    const spells = useGameDataStore(state => state.spells);
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions()

    const [selectedSpell, setSelectedSpell] = useState<string | undefined>(undefined);
    const [selectedMastery, setSelectedMastery] = useState<Mastery | undefined>(undefined);

    async function addSpell() {
        close();
        await invoke("add_creature_spell", {id: currentCreature?.id, spell: selectedSpell, mastery: selectedMastery})
            .then(() => {
                const updatedSpells = [...currentCreature?.known_spells.spells!, { spell: selectedSpell, mastery: selectedMastery}];
                const updatedModel = ObjectUtils.updateObjectDynamically(currentCreature!, "known_spells.spells", updatedSpells);
                actions.updateCreature(updatedModel);
            })
    }

    return (
    <>
        <Button radius={0} bg="green" onClick={open}>Add spell</Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Adding new spell to creature</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <Select
                            searchable
                            label="Spell"
                            size="xs"
                            radius={0}
                            value={selectedSpell}
                            data={
                                spells
                                    .filter(sp => !currentCreature?.known_spells.spells.some(spt => sp.game_id != selectedSpell && sp.game_id == spt.spell))
                                    .map(sp => ({label: sp.name, value: sp.game_id}))
                            }
                            onChange={(value) => setSelectedSpell(value!)}
                        />
                        <Select
                            label="Mastery"
                            size="xs"
                            radius={0}
                            value={selectedMastery}
                            data={[
                                {label: 'None', value: Mastery.None},
                                {label: 'Basic', value: Mastery.Basic},
                                {label: 'Advanced', value: Mastery.Advanced},
                                {label: 'Expert', value: Mastery.Expert},
                                {label: 'ExtraExpert', value: Mastery.ExtraExpert},
                            ]}
                            onChange={(value) => setSelectedMastery(value as Mastery)}
                        />
                        <Group justify="end">
                            <Button 
                                disabled={selectedSpell == undefined || selectedMastery == undefined}
                                radius={0}
                                bg="indigo"
                                onClick={() => addSpell()}
                            >Add spell</Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default CreatureSpellsEditor;