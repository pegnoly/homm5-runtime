import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, MultiSelect, Stack, Text, TextInput, Tooltip } from "@mantine/core";
import { Dialog } from "../types";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { IconPick } from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DialogGeneratorStore } from "../store";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../api";

export type DialogCreationPayload = {
    name: string,
    scriptName: string,
    directory: string,
    speakers: number []
}

function DialogCreator(params: {
    createdCallback: (value: Dialog) => void
}) {
    const availableSpeakers = DialogGeneratorStore.useSpeakers();

    const [opened, {open, close}] = useDisclosure(false);
    const [name, setName] = useState<string | undefined>(undefined);
    const [scriptName, setScriptName] = useState<string | undefined>(undefined);
    const [directory, setDirectory] = useState<string | undefined>(undefined);
    const [speakers, setSpeakers] = useState<number []>([]);

    async function pickDirectory() {
        await invoke("pick_dialog_directory")
    }

    listen<string>("dialog_directory_picked", (e) => {
        setDirectory(e.payload)
    });

    const mutation = useMutation({
        mutationFn: async(payload: DialogCreationPayload) => {
            return DialogGeneratorApi.createDialog(payload);
        },
        onSuccess(data, _variables, _context) {
            setName(undefined);
            setScriptName(undefined);
            setDirectory(undefined);
            setSpeakers([]);
            close();
            params.createdCallback(data);
        },
    })

    return (
    <>
    <Tooltip disabled={availableSpeakers?.length! > 0} label="Невозможно создать диалог без персонажей">
        <Button disabled={availableSpeakers?.length == 0} onClick={open} size="sm" radius={0}>
            Создать диалог
        </Button>
    </Tooltip>
    <ModalRoot centered opened={opened} onClose={close}>
        <ModalOverlay/>
        <ModalContent>
            <ModalHeader>
                <ModalTitle>Создание диалога</ModalTitle>
                <ModalCloseButton/>
            </ModalHeader>
            <ModalBody>
                <Stack>
                    <TextInput
                        label="Имя диалога"
                        placeholder="Существует только в программе"
                        value={name}
                        onChange={(e) => setName(e.currentTarget.value)}
                    />
                    <TextInput
                        label="Скриптовое имя диалога"
                        placeholder="Это имя будет иметь класс диалога в скриптах"
                        value={scriptName}
                        onChange={(e) => setScriptName(e.currentTarget.value)}
                    />
                    <Button onClick={() => pickDirectory()} radius={0} rightSection={<IconPick/>}>
                        Выбрать папку для генерации диалога
                    </Button>
                    <Tooltip label={directory}>
                        <Text lineClamp={1}>{directory}</Text>
                    </Tooltip>
                    <MultiSelect
                        label="Укажите персонажей"
                        searchable
                        value={speakers.map(s => s.toString())}
                        onChange={(value) => setSpeakers(value.map(v => parseInt(v)))}
                        data={availableSpeakers!.map(speaker => ({
                            value: speaker.id.toString(), label: speaker.name
                        }))}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={name == undefined || scriptName == undefined || directory == undefined || speakers.length == 0}
                            radius={0} 
                            onClick={() => mutation.mutate({
                                directory: directory!, 
                                name: name!,
                                scriptName: scriptName!,
                                speakers: speakers
                            })}
                        >Создать</Button>
                    </Group>
                </Stack>
            </ModalBody>
        </ModalContent>
    </ModalRoot>
    </>
    )
}

export default DialogCreator;