import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, MultiSelect, Stack, Text, TextInput, Tooltip } from "@mantine/core";
import { Dialog } from "../types";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { IconPick } from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSpeakers } from "../store";
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
    const availableSpeakers = useSpeakers();

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
    <Tooltip disabled={availableSpeakers?.length! > 0} label="Can't create dialogs without speakers">
        <Button disabled={availableSpeakers?.length == 0} onClick={open} size="sm" radius={0}>
            Create new dialog
        </Button>
    </Tooltip>
    <ModalRoot centered opened={opened} onClose={close}>
        <ModalOverlay/>
        <ModalContent>
            <ModalHeader>
                <ModalTitle>New dialog creation</ModalTitle>
                <ModalCloseButton/>
            </ModalHeader>
            <ModalBody>
                <Stack>
                    <TextInput
                        label="Dialog name"
                        placeholder="Enter name"
                        value={name}
                        onChange={(e) => setName(e.currentTarget.value)}
                    />
                    <TextInput
                        label="Dialog script name"
                        placeholder="Enter name"
                        value={scriptName}
                        onChange={(e) => setScriptName(e.currentTarget.value)}
                    />
                    <Button onClick={() => pickDirectory()} radius={0} rightSection={<IconPick/>}>
                        Pick directory
                    </Button>
                    <Tooltip label={directory}>
                        <Text lineClamp={1}>{directory}</Text>
                    </Tooltip>
                    <MultiSelect
                        label="Dialog speakers"
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
                        >Create</Button>
                    </Group>
                </Stack>
            </ModalBody>
        </ModalContent>
    </ModalRoot>
    </>
    )
}

export default DialogCreator;