import { Button, Group, Select, Stack, TextInput } from "@mantine/core";
import { useState } from "react";
import { TownType } from "../fightGenerator/types";
import { invoke } from "@tauri-apps/api/core";

function HeroCreatorLayout() {
    const [name, setName] = useState<string | undefined>(undefined);
    const [scriptName, setScriptName] = useState<string | undefined>(undefined);
    const [town, setTown] = useState<TownType | undefined>(undefined);

    async function create() {
        await invoke("create_hero", {heroName: name, heroScriptName: scriptName, town: town});
        setName(undefined);
        setScriptName(undefined);
        setTown(undefined);
    }

    return (
    <div style={{display: 'flex', paddingLeft: '10%'}}>
        <Stack>
            <TextInput
                radius={0}
                label="Actual name"
                value={name}
                onChange={(e) => setName(e.currentTarget.value)}
            />
            <TextInput
                radius={0}
                label="Script name"
                value={scriptName}
                onChange={(e) => setScriptName(e.currentTarget.value)}
            />
            <Select
                radius={0}
                label="Town type"
                value={town}
                onChange={(value) => setTown(value as TownType)}
                data={[
                    {value: TownType.TownAcademy, label: "Academy"}, 
                    {value: TownType.TownDungeon, label: "Dungeon"},
                    {value: TownType.TownHeaven, label: "Heaven"},
                    {value: TownType.TownInferno, label: "Inferno"},
                    {value: TownType.TownFortress, label: "Fortress"},
                    {value: TownType.TownPreserve, label: "Preserve"},
                    {value: TownType.TownNecromancy, label: "Necromancy"},
                    {value: TownType.TownStronghold, label: "Stronghold"},
                    {value: TownType.TownNoType, label: "Neutral"},
                ]}
            />
            <Group justify="end">
                <Button 
                    onClick={create}
                    radius={0}
                    disabled={name == undefined || scriptName == undefined || town == undefined}
                >Create hero</Button>
            </Group>
        </Stack>
    </div>
    )
}

export default HeroCreatorLayout;