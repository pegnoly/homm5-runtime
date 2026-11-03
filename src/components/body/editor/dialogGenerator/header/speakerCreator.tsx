import { Button, Checkbox, ColorPicker, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { Speaker, SpeakerType, getSpeakerDisplayName } from "../types";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../api";
import { DialogGeneratorStore } from "../store";
import {useCurrentMapId} from "@/stores/common.ts";
import useGameDataStore, {HeroModel} from "@/stores/GameDataStore";
import {TownType} from "@/components/body/editor/fightGenerator/types";

export type CreateSpeakerPayload = {
    nameOverride: string | null,
    color: string,
    speakerType: SpeakerType,
    localToMap: number | null
}

type SpeakerKind = "NoSpeaker" | "Hero" | "Creature";

const townGroupNames: Record<TownType, string> = {
    [TownType.TownNoType]: "Нейтралы",
    [TownType.TownHeaven]: "Орден порядка",
    [TownType.TownInferno]: "Инферно",
    [TownType.TownNecromancy]: "Некрополис",
    [TownType.TownPreserve]: "Лесной союз",
    [TownType.TownDungeon]: "Лига теней",
    [TownType.TownAcademy]: "Академия волшебства",
    [TownType.TownFortress]: "Северные кланы",
    [TownType.TownStronghold]: "Великая орда",
    [TownType.TownSpecial]: "Специальный",
};

const townOrder: TownType[] = [
    TownType.TownHeaven,
    TownType.TownInferno,
    TownType.TownNecromancy,
    TownType.TownPreserve,
    TownType.TownDungeon,
    TownType.TownAcademy,
    TownType.TownFortress,
    TownType.TownStronghold,
    TownType.TownSpecial,
    TownType.TownNoType,
];

function heroDisplayName(hero: HeroModel): string {
    const name = hero.editable?.name?.replace(/^\uFEFF/, "").trim();
    return name || hero.script_name || hero.id;
}

function buildHeroSelectData(heroes: HeroModel[]) {
    return townOrder
        .map(town => ({
            group: townGroupNames[town],
            items: heroes
                .filter(hero => hero.town == town)
                .map(hero => ({ value: hero.script_name, label: heroDisplayName(hero) }))
                .sort((a, b) => a.label.localeCompare(b.label)),
        }))
        .filter(group => group.items.length > 0);
}

function SpeakerCreator(params: {
    createdCallback: (value: Speaker) => void
}) {

    const speakers = DialogGeneratorStore.useSpeakers();
    const creatures = useGameDataStore(state => state.creatures);
    const heroes = useGameDataStore(state => state.heroes);
    const currentMapId = useCurrentMapId();

    const creatureSelectData = creatures.map(creature => ({
        value: creature.id.toString(),
        label: creature.name,
    }));
    const heroSelectData = buildHeroSelectData(heroes);

    const [opened, {open, close}] = useDisclosure(false);
    const [nameOverride, setNameOverride] = useState<string | undefined>(undefined);
    const [speakerKind, setSpeakerKind] = useState<SpeakerKind | undefined>(undefined);
    const [speakerIdentifier, setSpeakerIdentifier] = useState<string>("");
    const [color, setColor] = useState<string | undefined>(undefined);
    const [isLocalToMap, setIsLocalToMap] = useState<boolean>(false);
    const [useColorFromExistingSpeaker, setUseColorFromExistingSpeaker] = useState<boolean>(false);

    function buildSpeakerType(): SpeakerType | undefined {
        switch (speakerKind) {
            case "NoSpeaker":
                return "NoSpeaker";
            case "Hero": {
                const heroName = speakerIdentifier.trim();
                return heroName.length > 0 ? { Hero: heroName } : undefined;
            }
            case "Creature": {
                const creatureId = Number(speakerIdentifier);
                return speakerIdentifier.trim().length > 0 && Number.isInteger(creatureId) && creatureId > 0
                    ? { Creature: creatureId }
                    : undefined;
            }
            default:
                return undefined;
        }
    }

    const speakerType = buildSpeakerType();

    const mutation = useMutation({
        mutationFn: async(payload: CreateSpeakerPayload) => {
            return DialogGeneratorApi.createSpeaker(payload);
        },
        onSuccess(data, _variables, _context) {
            setNameOverride(undefined);
            setSpeakerKind(undefined);
            setSpeakerIdentifier("");
            setColor(undefined);
            setIsLocalToMap(false);
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
                        label="Имя персонажа (необязательно)"
                        placeholder="Переопределяет имя из данных игры"
                        value={nameOverride}
                        onChange={(e) => setNameOverride(e.currentTarget.value)}
                    />
                    <Select
                        label="Тип персонажа"
                        placeholder="Выбрать тип"
                        value={speakerKind}
                        onChange={(value) => {
                            setSpeakerKind(value as SpeakerKind | undefined);
                            setSpeakerIdentifier("");
                        }}
                        data={[
                            { label: "No speaker", value: "NoSpeaker" },
                            { label: "Hero", value: "Hero" },
                            { label: "Creature", value: "Creature" },
                        ]}
                    />
                    {
                        speakerKind == "Hero" &&
                        <Select
                            searchable
                            label="Герой"
                            placeholder="Выберите героя"
                            nothingFoundMessage="Герои не найдены"
                            value={speakerIdentifier || null}
                            onChange={(value) => setSpeakerIdentifier(value ?? "")}
                            data={heroSelectData}
                        />
                    }
                    {
                        speakerKind == "Creature" &&
                        <Select
                            searchable
                            label="Существо"
                            placeholder="Выберите существо"
                            nothingFoundMessage="Существа не найдены"
                            value={speakerIdentifier || null}
                            onChange={(value) => setSpeakerIdentifier(value ?? "")}
                            data={creatureSelectData}
                        />
                    }
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
                            label: `${getSpeakerDisplayName(s)} [${s.color}]`, value: JSON.stringify(s)
                        }))}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={speakerType == undefined || color == undefined}
                            radius={0} 
                            onClick={() => mutation.mutate({
                                nameOverride: nameOverride?.trim() || null,
                                color: color!,
                                speakerType: speakerType!,
                                localToMap: isLocalToMap ? currentMapId! : null
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
