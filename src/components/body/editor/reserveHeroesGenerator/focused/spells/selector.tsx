import { useState } from "react";
import { MagicSchool, SpellData } from "../../types";
import { ReserveHeroesGenerator } from "../../store";
import { ActionIcon, Select } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

function ReserveHeroSpellsSelector({school}: { school: MagicSchool }) {
    const [spells, setSpells] = useState<SpellData[]>([]);
    const [selectedSpell, setSelectedSpell] = useState<string | undefined>(undefined);

    const id = ReserveHeroesGenerator.useCurrentId();
    const actions = ReserveHeroesGenerator.useActions();

    async function addSpell() {
        await invoke("add_spell", {id: id, spell: selectedSpell});
        actions.addSpell(selectedSpell!)
        setSelectedSpell(undefined);
    }

    return (
    <>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-around', alignItems: 'center'}}>
            <Select
                size="xs"
                radius={0}
                label="Select spell"
                value={selectedSpell}
                onChange={(value) => setSelectedSpell(value!)}
                data={spells.map(s => ({
                    value: s.game_id,
                    label: s.name
                }))}
            />
            <ActionIcon
                disabled={selectedSpell == undefined}
                onClick={addSpell}>
                <IconPlus/>
            </ActionIcon>
        </div>
        <SpellsLoader school={school} onLoad={setSpells}/>
    </>
    )
}

function useSpells(school: MagicSchool) {
    return useQuery({
        queryKey: ["spells", school],
        queryFn: async() => {
            return invoke<SpellData[]>("load_spells", {school: school});
        }
    })
}

function SpellsLoader({school, onLoad}: { school: MagicSchool, onLoad: (values: SpellData[]) => void }) {
    const { data } = useSpells(school);
    if (data != undefined) {
        onLoad(data);
    }
    return null;
}

export default ReserveHeroSpellsSelector;