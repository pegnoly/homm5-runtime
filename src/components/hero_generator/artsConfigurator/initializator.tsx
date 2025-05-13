import { useState } from "react";
import { AssetGenerationType, HeroAssetArtifactsModel } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { Button, Modal, Select, Space } from "antd";

function HeroAssetArtifactsInitializator(params: {assetId: number, initializedCallback: (value: HeroAssetArtifactsModel) => void}) {
    const [opened, setOpened] = useState<boolean>(false);
    const [generationType, setGenerationType] = useState<AssetGenerationType | null>(null);

    async function close() {
        setOpened(false);
    }

    async function create() {
        close();
        await invoke<HeroAssetArtifactsModel>("create_artifacts_data_for_asset", {assetId: params.assetId, generationType: generationType})
            .then((value) => params.initializedCallback(value));
    }

    return <>
        <Button onClick={() => setOpened(true)}>Init artifacts for asset</Button>
        <Modal
            open={opened}
            onCancel={close}
            onClose={close}
        >
            <Space>
                <Select value={generationType} onChange={setGenerationType} placeholder="Select artifacts generation type">
                    <Select.Option key={0} value={AssetGenerationType.Static}>Static generation</Select.Option>
                    <Select.Option key={1} value={AssetGenerationType.Dynamic}>Dynamic generation</Select.Option>
                </Select>
                <Button disabled={!generationType} onClick={create}>Create</Button>
            </Space>
        </Modal>
    </>
}

export default HeroAssetArtifactsInitializator;