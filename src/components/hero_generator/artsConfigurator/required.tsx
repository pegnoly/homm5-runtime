import { Button, List, Select, Space, Typography } from "antd";
import { RequiredArtifacts } from "./types";
import useGameDataStore from "../../../stores/GameDataStore";
import { useState } from "react";
import { PlusOutlined } from "@ant-design/icons";
import ArtifactListItem from "./item";
import { invoke } from "@tauri-apps/api/core";

function RequiredArtifactsList(params: {
    modelId: number,
    currentArtifacts: RequiredArtifacts, 
    updateCallback: (values: RequiredArtifacts) => void}
) {
    const availableArtifacts = useGameDataStore((state) => state.artifacts);
    const [selectedId, setSelectedId] = useState<number | null>(null);

    async function addArtifact(artId: number) {
        setSelectedId(null);
        params.updateCallback({...params.currentArtifacts, ids: [...params.currentArtifacts.ids, artId]});
        invoke("add_required_artifact", {assetId: params.modelId, artifactId: artId});
    }

    async function removeArtifact(artId: number) {
        const updatedArtifacts = params.currentArtifacts.ids.filter(id => id != artId);
        params.updateCallback({...params.currentArtifacts, ids: updatedArtifacts});
        invoke("remove_required_artifact", {assetId: params.modelId, artifactId: artId});
    }

    return <div style={{display: 'flex', flexDirection: 'column', justifyItems: 'start'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Required artifacts</Typography.Text>
        <Space>
            <Select 
                showSearch 
                style={{width: 150}} 
                value={selectedId} 
                onChange={setSelectedId}
                filterOption={(input, option) =>
                    (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
                }
                options={availableArtifacts.filter(art => !params.currentArtifacts.ids.includes(art.id)).map((art, _index) => ({value: art.id, label: art.name}))}
            />
            <Button disabled={!selectedId} icon={<PlusOutlined/>} onClick={() => addArtifact(selectedId!)}/>
        </Space>
        <List>{params.currentArtifacts.ids.map((art, index) => (
            <List.Item key={index}>
                <ArtifactListItem artifact={availableArtifacts.find(value => value.id == art)!} removeArtifactCallback={removeArtifact}/>
            </List.Item>
        ))}</List>
    </div>
}

export default RequiredArtifactsList;