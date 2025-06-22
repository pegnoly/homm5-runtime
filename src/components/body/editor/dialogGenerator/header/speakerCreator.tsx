import { Button, ColorPicker, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { Speaker, SpeakerType } from "../types";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../api";

export type CreateSpeakerPayload = {
    name: string,
    scriptName: string,
    color: string,
    speakerType: SpeakerType
}

function SpeakerCreator(params: {
    createdCallback: (value: Speaker) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [name, setName] = useState<string | undefined>(undefined);
    const [scriptName, setScriptName] = useState<string | undefined>(undefined);
    const [type, setType] = useState<SpeakerType | undefined>(undefined);
    const [color, setColor] = useState<string | undefined>(undefined);

    const mutation = useMutation({
        mutationFn: async(payload: CreateSpeakerPayload) => {
            return DialogGeneratorApi.createSpeaker(payload);
        },
        onSuccess(data, _variables, _context) {
            setName(undefined);
            setScriptName(undefined);
            setColor(undefined);
            setType(undefined);
            close();
            params.createdCallback(data);
        },
    })

    return (
    <>
    <Button onClick={open} radius={0} bg="cyan">
        Create new speaker
    </Button>
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
                        label="Speaker name"
                        placeholder="Enter name"
                        value={name}
                        onChange={(e) => setName(e.currentTarget.value)}
                    />
                    <TextInput
                        label="Speaker script name"
                        placeholder="Enter name"
                        value={scriptName}
                        onChange={(e) => setScriptName(e.currentTarget.value)}
                    />
                    <Select
                        label="Speaker type"
                        placeholder="Select type"
                        value={type}
                        onChange={(value) => setType(value as SpeakerType)}
                        data={[
                            { label: "Hero", value: SpeakerType.Hero},
                            { label: "Creature", value: SpeakerType.Creature},
                        ]}
                    />
                    <ColorPicker
                        format="hex"
                        value={color}
                        onChange={setColor}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={name == undefined || scriptName == undefined || color == undefined || type == undefined}
                            radius={0} 
                            onClick={() => mutation.mutate({
                                name: name!,
                                scriptName: scriptName!,
                                color: color!,
                                speakerType: type!
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

export default SpeakerCreator;