import useGameDataStore from "@/stores/GameDataStore";
import { Button, ButtonGroup, Group, NumberInput, Select } from "@mantine/core";
import { ArtifactEditorStore } from "./store";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function ArtifactEditorHeader() {
    const artifacts = useGameDataStore(state => state.artifacts);
    const actions = ArtifactEditorStore.useActions();

    const [selectedId, setSelectedId] = useState<number | undefined>(undefined);

    return <div style={{width: '100%', display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
        <Group>
            <Select
                label="Select by name"
                radius={0}
                data={artifacts.map(a => ({value: a.id.toString(), label: a.name}))}
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
                    onClick={() => actions.loadCurrent(artifacts.find(a => a.id == selectedId)!)}
                >Load</Button>
                <Button
                    bg="gray"
                    radius={0}
                    onClick={() => invoke("rebuild_artifacts_file")}
                >Build artifacts.xdb</Button>
            </ButtonGroup>
        </Group>
    </div>
}

export default ArtifactEditorHeader;