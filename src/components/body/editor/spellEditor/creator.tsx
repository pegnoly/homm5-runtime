import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, Text, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useState } from "react";
import { MagicSchool } from "../reserveHeroesGenerator/types";
import useGameDataStore, { SpellModel } from "@/stores/GameDataStore";
import { SpellEditorStore } from "./store";

function SpellCreator() {
    const updateSpells = useGameDataStore(state => state.update_spells);
    const actions = SpellEditorStore.useActions();

    const [opened, {open, close}] = useDisclosure(false);

    const [gameId, setGameId] = useState<string>("");
    const [name, setName] = useState<string>("");
    const [desc, setDesc] = useState<string>("");
    const [textsPath, setTextsPath] = useState<string>("");
    const [iconPath, setIconPath] = useState<string>("");
    const [school, setSchool] = useState<MagicSchool>(MagicSchool.Adventure); 

    listen<string>("spell_texts_directory_picked", e => {
        setTextsPath(e.payload);
    });

    listen<string>("spell_icon_directory_picked", e => {
        console.log("??? ", e.payload)
        setIconPath(e.payload);
    });

    async function create() {
        await invoke<SpellModel>("create_new_spell", {gameId: gameId, name: name, desc: desc, textsPath: textsPath, iconPath: iconPath, school: school})
            .then((value) => {
                actions.updateCurrent(value);
                updateSpells(value);
            });
        close();
    }

    return <>
        <Button
            radius={0}
            onClick={open}
        >Create new spell</Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>New spell creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <TextInput
                            radius={0}
                            label="Spell game id"
                            value={gameId}
                            onChange={(value) => setGameId(value.currentTarget.value)}
                        />
                        <TextInput
                            radius={0}
                            label="Spell name"
                            value={name}
                            onChange={(value) => setName(value.currentTarget.value)}
                        />
                        <TextInput
                            radius={0}
                            label="Spell desc"
                            value={desc}
                            onChange={(value) => setDesc(value.currentTarget.value)}
                        />
                        <Group align="center">
                            <Text>{textsPath}</Text>
                            <Button
                                radius={0}
                                onClick={() => invoke("pick_spell_texts_directory")}
                            >Pick texts dir</Button>
                        </Group>
                        <Group align="center">
                            <Text>{iconPath}</Text>
                            <Button
                                radius={0}
                                onClick={() => invoke("pick_spell_icon_directory")}
                            >Pick icon dir</Button>
                        </Group>
                        <Select
                            radius={0}
                            label="Select school"
                            data={[
                                {label: "Destructive", value: MagicSchool.Destructive},
                                {label: "Dark", value: MagicSchool.Dark},
                                {label: "Light", value: MagicSchool.Light},
                                {label: "Summoning", value: MagicSchool.Summoning},
                                {label: "Warcries", value: MagicSchool.Warcries},
                                {label: "Adventure", value: MagicSchool.Adventure},
                                {label: "Special", value: MagicSchool.Special},
                                {label: "Runic", value: MagicSchool.Runic}
                            ]}
                            value={school}
                            onChange={(value) => setSchool(value as MagicSchool)}
                        />
                        <Group justify="end">
                            <Button 
                                radius={0}
                                disabled={
                                    gameId.length == 0 ||
                                    name.length == 0 ||
                                    desc.length == 0 ||
                                    textsPath.length == 0 ||
                                    iconPath.length == 0
                                }
                                onClick={create}
                            >
                                Create spell
                            </Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
}

export default SpellCreator;