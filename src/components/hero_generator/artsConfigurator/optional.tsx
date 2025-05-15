import { Button, List, Segmented, Select, Space, Typography } from "antd";
import { ArtifactSlotType, OptionalArtifacts } from "./types";
import { useState } from "react";
import useGameDataStore from "../../../stores/GameDataStore";
import { PlusOutlined } from "@ant-design/icons";
import ArtifactListItem from "./item";
import { invoke } from "@tauri-apps/api/core";

function OptionalArtifactsList(params: {modelId: number, currentArtifacts: OptionalArtifacts, updateCallback: (value: OptionalArtifacts) => void}) {
    const [slot, setSlot] = useState<ArtifactSlotType>(ArtifactSlotType.Primary);
    const [selectedId, setSelectedId] = useState<number | null>(null);

    const availableArtifacts = useGameDataStore((state) => state.artifacts);

    async function addArtifact(artId: number) {
        setSelectedId(null);
        params.currentArtifacts.values[slot].push(artId);
        params.updateCallback(params.currentArtifacts);
        invoke("add_optional_artifact", {assetId: params.modelId, slot: slot, artifactId: artId});
    }

    async function removeArtifact(artId: number) {
        params.currentArtifacts.values[slot] = params.currentArtifacts.values[slot].filter(art => art != artId);
        params.updateCallback(params.currentArtifacts);
        invoke("remove_optional_artifact", {assetId: params.modelId, slot: slot, artifactId: artId});
    }

    return <div style={{display: 'flex', flexDirection: 'column'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Optional artifacts</Typography.Text>
        <div style={{width: '100%', height: '100%'}}>
            <div style={{display: 'flex', flexDirection: 'row', gap: '5%'}}>
                <Segmented
                    value={slot}
                    onChange={(value) => {
                        setSelectedId(null);
                        setSlot(value);
                    }}
                    vertical
                    options={[
                        {value: ArtifactSlotType.Chest, label: "Chest"},
                        {value: ArtifactSlotType.Neck, label: "Neck"},
                        {value: ArtifactSlotType.Miscslot1, label: "Pocket"},
                        {value: ArtifactSlotType.Head, label: "Head"},
                        {value: ArtifactSlotType.Primary, label: "Primary"},
                        {value: ArtifactSlotType.Secondary, label: "Secondary"},
                        {value: ArtifactSlotType.Shoulders, label: "Shoulders"},
                        {value: ArtifactSlotType.Feet, label: "Feet"},
                        {value: ArtifactSlotType.Finger, label: "Finger"},
                        {value: ArtifactSlotType.Inventory, label: "Inventory"},
                    ]}
                />
                <div style={{display: 'flex', flexDirection: 'column', justifyItems: 'self-start'}}>
                    <Space>
                        <Select
                            showSearch 
                            value={selectedId} 
                            onChange={setSelectedId}
                            style={{width: 150}}
                            options={
                                availableArtifacts
                                    .filter(art => art.slot == slot && !params.currentArtifacts.values[slot].includes(art.id))
                                    .map((art, _index) => ({value: art.id, label: art.name}))
                            }
                            filterOption={(input, option) =>
                                (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
                            }
                        />
                        <Button disabled={!selectedId} icon={<PlusOutlined/>} onClick={() => addArtifact(selectedId!)}/>
                    </Space>
                    {
                        params.currentArtifacts.values[slot].length > 0 ?
                        <div>
                            <List>{params.currentArtifacts.values[slot].map((art, index) => (
                                <List.Item key={index}>
                                    <ArtifactListItem 
                                        artifact={
                                            availableArtifacts
                                                .find(model => model.id == art)!
                                        } 
                                        removeArtifactCallback={removeArtifact}
                                    />
                                </List.Item>
                            ))}</List>
                        </div> :
                        null
                    }
                </div>
            </div>
        </div>
    </div> 
}

export default OptionalArtifactsList;