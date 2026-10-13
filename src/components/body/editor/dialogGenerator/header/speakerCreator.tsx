import { Button, Checkbox, ColorPicker, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { Speaker, SpeakerType } from "../types";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../api";
import { useSpeakers } from "../store";
import {useCurrentMapId} from "@/stores/common.ts";

export type CreateSpeakerPayload = {
    name: string,
    scriptName: string,
    color: string,
    speakerType: SpeakerType,
    local_to_map: number | null
}

function SpeakerCreator(params: {
    createdCallback: (value: Speaker) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [name, setName] = useState<string | undefined>(undefined);
    const [scriptName, setScriptName] = useState<string | undefined>(undefined);
    const [type, setType] = useState<SpeakerType | undefined>(undefined);
    const [color, setColor] = useState<string | undefined>(undefined);
    const [isLocalToMap, setIsLocalToMap] = useState<boolean>(false);

    const speakers = useSpeakers();
    const currentMapId = useCurrentMapId();
    const [useColorFromExistingSpeaker, setUseColorFromExistingSpeaker] = useState<boolean>(false);

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
        Создать персонажа
    </Button>
    <ModalRoot centered opened={opened} onClose={close}>
        <ModalOverlay/>
        <ModalContent>
            <ModalHeader>
                <ModalTitle>Создание нового персонажа</ModalTitle>
                <ModalCloseButton/>
            </ModalHeader>
            <ModalBody>
                <Stack>
                    <TextInput
                        label="Имя персонажа"
                        placeholder="То, которое будет отображаться в игре"
                        value={name}
                        onChange={(e) => setName(e.currentTarget.value)}
                    />
                    <TextInput
                        label="Скриптовое имя персонажа"
                        placeholder="Для персонажей юнитов можно использовать как цифры, так и константы"
                        value={scriptName}
                        onChange={(e) => setScriptName(e.currentTarget.value)}
                    />
                    <Select
                        label="Тип персонажа"
                        placeholder="Выбрать тип"
                        value={type}
                        onChange={(value) => setType(value as SpeakerType)}
                        data={[
                            { label: "Hero", value: SpeakerType.Hero},
                            { label: "Creature", value: SpeakerType.Creature},
                        ]}
                    />
                    <Checkbox
                        radius={0}
                        label="Локальный для текущей карты?"
                        checked={isLocalToMap}
                        onChange={(e) => setIsLocalToMap(e.currentTarget.checked)}
                    />
                    <ColorPicker
                        format="hex"
                        value={color}
                        onChange={setColor}
                    />
                    <Checkbox
                        label="Использовать цвет существующего персонажа?"
                        checked={useColorFromExistingSpeaker} 
                        onChange={(e) => setUseColorFromExistingSpeaker(e.currentTarget.checked)}/>
                    <Select
                        searchable
                        disabled={!useColorFromExistingSpeaker}
                        value={color}
                        onChange={(e) => {
                            const speaker: Speaker = JSON.parse(e!);
                            setColor(speaker.color)
                        }}
                        data={speakers?.map(s => ({
                            label: `${s.name} [${s.color}]`, value: JSON.stringify(s)
                        }))}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={name == undefined || scriptName == undefined || color == undefined || type == undefined}
                            radius={0} 
                            onClick={() => mutation.mutate({
                                name: name!,
                                scriptName: scriptName!,
                                color: color!,
                                speakerType: type!,
                                local_to_map: isLocalToMap ? currentMapId! : null
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

export default SpeakerCreator;