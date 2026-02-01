import EditableProperty from "@/components/common/editableProperty";
import { ArtifactEditorStore } from "./store";
import { Button, Group, Select, Stack, Textarea, TextInput } from "@mantine/core";
import { ArtifactClassType, ArtifactSlotType } from "../fightGenerator/elements/artifactsGenerator/types";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";
import { useEffect, useState } from "react";
import ArtifactIconSelector from "./iconSelector";
import useGameDataStore from "@/stores/GameDataStore";
import ArtifactPathsSelector from "./pathsSelector";

function ArtifactEditorBody() {
    const current = ArtifactEditorStore.useCurrent();
    const action = ArtifactEditorStore.useActions();

    const updateArtifacts = useGameDataStore(state => state.update_artifacts);

    const [localName, setLocalName] = useState<string | undefined>("");
    const [nameEditable, setNameEditable] = useState<boolean>(false);

    const [localDesc, setLocalDesc] = useState<string | undefined>("");
    const [descEditable, setDescEditable] = useState<boolean>(false);

    useEffect(() => {
        if (current != undefined) {
            setLocalName(current.name)
            setLocalDesc(current.desc)
        }
    }, [current])
    
    async function updateStat(stat: string, value: number) {
        await invoke(`update_artefact_${stat}`, {id: current?.id, value: value})
            .then(() => {
                const updated = ObjectUtils.updateObjectDynamically(current!, stat, value);
                action.updateCurrent(updated);
                updateArtifacts(updated);
            })
    }

    async function updateSlot(value: ArtifactSlotType) {
        await invoke("update_artefact_slot", {id: current?.id, value: value})
            .then(() => {
                const updated = ObjectUtils.updateObjectDynamically(current!, "slot", value);
                action.updateCurrent(updated);
                updateArtifacts(updated);
            })
    }

    async function updateClass(value: ArtifactClassType) {
        await invoke("update_artefact_class", {id: current?.id, value: value})
            .then(() => {
                const updated = ObjectUtils.updateObjectDynamically(current!, "class", value);
                action.updateCurrent(updated);
                updateArtifacts(updated);
            })
    }

    async function saveName() {
        await invoke("update_artefact_name", {id: current?.id, value: localName, path: current?.name_txt})
            .then(() => {
                const updated = ObjectUtils.updateObjectDynamically(current!, "name", localName);
                action.updateCurrent(updated);
                updateArtifacts(updated);
            }) 
    }

    async function saveDesc() {
        await invoke("update_artefact_desc", {id: current?.id, value: localDesc, path: current?.desc_txt})
            .then(() => {
                const updated = ObjectUtils.updateObjectDynamically(current!, "desc", localDesc);
                action.updateCurrent(updated);
                updateArtifacts(updated);
            })
    }

    return <>
        {
            current == undefined ? null :
            <div style={{width: '100%', display: 'flex', flexDirection: 'row', gap: '2%'}}>
                <div style={{width: '25%', paddingTop: '3%'}}>
                    <Stack align="center">
                        <Select
                            radius={0}
                            label="Slot"
                            data={[
                                {value: ArtifactSlotType.Chest, label: "Chest"},
                                {value: ArtifactSlotType.Feet, label: "Feet"},
                                {value: ArtifactSlotType.Finger, label: "Finger"},
                                {value: ArtifactSlotType.Head, label: "Head"},
                                {value: ArtifactSlotType.Inventory, label: "Inventory"},
                                {value: ArtifactSlotType.Miscslot1, label: "Pocket"},
                                {value: ArtifactSlotType.Neck, label: "Neck"},
                                {value: ArtifactSlotType.Primary, label: "Primary"},
                                {value: ArtifactSlotType.Secondary, label: "Secondary"},
                                {value: ArtifactSlotType.Shoulders, label: "Shoulders"}
                            ]}
                            value={current.slot}
                            onChange={(value) => updateSlot(value as ArtifactSlotType)}
                        />
                        <Select
                            radius={0}
                            label="Class"
                            data={[
                                {value: ArtifactClassType.Minor, label: "Minor"},
                                {value: ArtifactClassType.Major, label: "Major"},
                                {value: ArtifactClassType.Relic, label: "Relic"},
                                {value: ArtifactClassType.Grail, label: "Grail"}
                            ]}
                            value={current.class}
                            onChange={(value) => updateClass(value as ArtifactClassType)}
                        />
                        <EditableProperty
                            initialValue={current.attack}
                            label="Attack"
                            onSave={(value) => updateStat("attack", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.defence}
                            label="Defence"
                            onSave={(value) => updateStat("defence", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.spell_power}
                            label="Spellpower"
                            onSave={(value) => updateStat("spell_power", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.knowledge}
                            label="Knowledge"
                            onSave={(value) => updateStat("knowledge", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.luck}
                            label="Luck"
                            onSave={(value) => updateStat("luck", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.morale}
                            label="Morale"
                            onSave={(value) => updateStat("morale", value as number)}
                        />
                        <EditableProperty
                            initialValue={current.cost}
                            label="Cost"
                            onSave={(value) => updateStat("cost", value as number)}
                        />
                    </Stack>
                </div>
                <div style={{width: '72%', alignContent: 'center', paddingTop: '3%', display: 'flex', flexDirection: 'column', gap: '2%'}}>
                    <ArtifactPathsSelector/>
                    <Group maw={550}>
                        <TextInput 
                            disabled={!nameEditable} 
                            value={localName}
                            onChange={(value) => setLocalName(value.currentTarget.value)}
                        />
                        <Button 
                            radius={0}
                            onClick={() => {
                                if (nameEditable == true) {
                                    setNameEditable(false)
                                    saveName()
                                } else {
                                    setNameEditable(true)
                                }
                            }}
                        >{nameEditable ? "Save name" : "Edit name"}</Button>
                    </Group>
                    <Stack>
                        <Textarea
                            disabled={!descEditable}
                            rows={12}
                            value={localDesc}
                            onChange={(value) => setLocalDesc(value.currentTarget.value)}
                        />
                        <Group justify="end">
                            <Button 
                                radius={0}
                                onClick={() => {
                                    if (descEditable == true) {
                                        setDescEditable(false)
                                        saveDesc()
                                    } else {
                                        setDescEditable(true)
                                    }
                                }}
                            >{descEditable ? "Save desc" : "Edit desc"}</Button>
                        </Group>
                    </Stack>
                    <ArtifactIconSelector/>
                </div>
            </div>
        }
    </> 
}

export default ArtifactEditorBody;