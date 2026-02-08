import useGameDataStore from "@/stores/GameDataStore";
import { Button, ButtonGroup, Group, NumberInput, Select } from "@mantine/core";
import { SpellEditorStore } from "./store";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import SpellCreator from "./creator";

function SpellEditorHeader() {
    const spells = useGameDataStore(state => state.spells);
    const current = SpellEditorStore.useCurrent();
    const actions = SpellEditorStore.useActions();

    const [selectedId, setSelectedId] = useState<number | undefined>(undefined);

    return <div style={{width: '100%', display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
        <Group>
            <Select
                label="Select by name"
                radius={0}
                data={spells.map(s => ({value: s.id.toString(), label: s.name}))}
                value={selectedId?.toString()}
                onChange={(value) => setSelectedId(parseInt(value!))}
                searchable
            />
            <NumberInput
                label="Select by id"
                radius={0}
                min={1}
                max={500}
                value={selectedId}
                onChange={(value) => setSelectedId(value as number)}
            />
            <ButtonGroup>
                <Button
                    bg="cyan"
                    radius={0}
                    disabled={selectedId == undefined}
                    onClick={() => actions.loadCurrent(spells.find(s => s.id == selectedId)!)}
                >Load</Button>
                <Button
                    bg="gray"
                    radius={0}
                    disabled={current == undefined}
                    onClick={() => invoke("generate_spell_file", {id: current?.id})}
                >Generate spell xdb</Button>
            </ButtonGroup>
            <SpellCreator/>
        </Group>
    </div>
}

export default SpellEditorHeader;