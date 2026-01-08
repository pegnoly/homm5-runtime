import useGameDataStore from "@/stores/GameDataStore";
import { Button, Group, NumberInput, Select } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { CreatureEditableModel } from "../types";
import { CreatureEditorStore } from "../store";

function CreatureToEditSelector() {
    const creatures = useGameDataStore(state => state.creatures)
    const actions = CreatureEditorStore.useActions();

    const [selectedId, setSelectedId] = useState<number | undefined>(undefined);

    const mutation = useMutation({
        mutationKey: ["creature_data", selectedId],
        mutationFn: async(id: number) => {
            return invoke<CreatureEditableModel>("load_creature", {id: id});
        },
        onSuccess(data, _variables, _context) {
            actions.loadCreature(data);
        },
    })

    return (
    <>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-around', alignItems: 'center', gap: '5%'}}>
            <Group align="center" justify="center">
                <Select
                    radius={0}
                    label="Select by name"
                    size="sm"
                    value={selectedId?.toString()}
                    onChange={(value) => setSelectedId(parseInt(value!))}
                    searchable
                    data={creatures.map(c => ({
                        label: c.inner_name != null ? c.inner_name : c.name,
                        value: c.id.toString()
                    }))}
                />
                <NumberInput
                    radius={0}
                    label="Select by id"
                    value={selectedId}
                    onChange={(value) => {
                        if (typeof(value) === "string") {
                            setSelectedId(parseInt(value))
                        } else {
                            setSelectedId(value)
                        }
                    }}
                />
                <Button 
                    radius={0} 
                    disabled={selectedId == undefined}
                    onClick={() => mutation.mutate(selectedId!)}
                >Load</Button>
            </Group>
        </div>
    </>
    )
}

export default CreatureToEditSelector;