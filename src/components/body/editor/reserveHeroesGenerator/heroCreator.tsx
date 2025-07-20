import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { TownType } from "../fightGenerator/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useCurrentMapId } from "@/stores/common";
import { ReservedHero } from "./types";

function ReserveHeroesGeneratorHeroCreator({player}: {player: number}) {
    const [opened, {open, close}] = useDisclosure(false);

    const mapId = useCurrentMapId();
 
    const [name, setName] = useState<string | undefined>(undefined);
    const [xdb, setXdb] = useState<string | undefined>(undefined);

    async function create() {
        await invoke<ReservedHero[]>("init_new_hero", {mapId: mapId, playerId: player, name: name, xdb: xdb});
        close();
    }

    return (
    <>
        <Button radius={0} onClick={open}>
            Create new hero
        </Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Reserve hero creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <TextInput 
                            label="Hero name"
                            value={name}
                            onChange={(e) => setName(e.currentTarget.value)}
                        />
                        <HeroSelector current={xdb} onSelected={setXdb}/>
                        <Group justify="md">
                            <Button 
                                radius={0}
                                disabled={name == undefined || xdb == undefined}
                                onClick={create}
                            >
                                Create
                            </Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

function HeroSelector({current, onSelected}: {
    current: string | undefined,
    onSelected: (value: string) => void
}) {
    const [town, setTown] = useState<TownType | undefined>(TownType.TownInferno);

    return (
    <Group>
        <Select
            label="Select hero town"
            value={town}
            onChange={(value) => setTown(value as TownType)}
            data={[
                {value: TownType.TownNoType, label: "Neutral"},
                {value: TownType.TownHeaven, label: "Haven"},
                {value: TownType.TownInferno, label: "Inferno"},
                {value: TownType.TownNecromancy, label: "Necropolis"},
                {value: TownType.TownPreserve, label: "Preserve"},
                {value: TownType.TownDungeon, label: "Dungeon"},
                {value: TownType.TownAcademy, label: "Academy"},
                {value: TownType.TownFortress, label: "Fortress"},
                {value: TownType.TownStronghold, label: "Stronghold"}
            ]}
        />
        <XdbSelector current={current} onSelected={onSelected} town={town!}/>
    </Group>
    )
}

type HeroXdb = {
    id: string,
    script_name: string
}

function useHeroXdbs(town: TownType) {
    return useQuery({
        queryKey: ['heroes_xdbs', town],
        queryFn: async() => {
            return invoke<HeroXdb[]>("load_heroes_data", {town: town});
        }
    });
}

function XdbSelector({town, current, onSelected}: {
    town: TownType,
    current: string | undefined,
    onSelected: (value: string) => void
}) {
    const { data } = useHeroXdbs(town);
    if (data == undefined) {
        return null;
    }

    return (
    <Select
        label="Select hero xdb"
        searchable
        value={current}
        onChange={(value) => onSelected(value!)}
        data={data!.map(h => ({
            value: h.id, label: h.script_name
        }))}
    />
    )
}

export default ReserveHeroesGeneratorHeroCreator;