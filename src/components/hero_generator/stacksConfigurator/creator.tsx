import { useState } from "react";
import { StackCountGenerationType, StackUnitGenerationType } from "./main";
import { AssetGenerationType } from "../artsConfigurator/types";
import { invoke } from "@tauri-apps/api/core";
import { Button, Modal, Select, Space } from "antd";
import { PlusOutlined } from "@ant-design/icons";

function HeroAssetStackCreator(params: {
    disabled: boolean,
    assetId: number, 
    createCallback: (createdId: number) => void
}) {
    const [opened, setOpened] = useState<boolean>(false);
    const [selectedUnitGenerationType, setSelectedUnitGenerationType] = useState<StackUnitGenerationType | null>(null);
    const [selectedCountGenerationType, setSelectedCountGenerationType] = useState<StackCountGenerationType | null>(null);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(null);

    async function create() {
        await close();
        await invoke<number>("create_stack", {
            assetId: params.assetId, 
            typeGenerationMode: selectedUnitGenerationType, 
            countGenerationMode: selectedCountGenerationType,
            generationType: selectedType
        })
        .then((value) => params.createCallback(value));
    }

    async function close() {
        setOpened(false);
    }

    return <>
        <Button 
            disabled={params.disabled}
            onClick={() => setOpened(true)}
            icon={<PlusOutlined/>}
        />
        <Modal
            open={opened}
            onClose={close}
            onCancel={close}
            onOk={create}
            centered
        >
            <Space direction="vertical">
                <Select 
                    value={selectedUnitGenerationType} 
                    onChange={setSelectedUnitGenerationType} 
                    placeholder="Select generation mode of stack unit type"
                >
                    <Select.Option value={StackUnitGenerationType.ConcreteUnit}>Generate concrete unit in stack</Select.Option>
                    <Select.Option value={StackUnitGenerationType.TierSlotBased}>Generate unit from given town and tier</Select.Option>
                </Select>
                <Select 
                    value={selectedCountGenerationType} 
                    onChange={setSelectedCountGenerationType} 
                    placeholder="Select generation mode of stack unit count type"
                >
                    <Select.Option value={StackCountGenerationType.Raw}>Use raw values for unit count</Select.Option>
                    <Select.Option value={StackCountGenerationType.PowerBased}>Generate counts using unit's power</Select.Option>
                </Select>
                <Select 
                    disabled={!selectedCountGenerationType || selectedCountGenerationType == StackCountGenerationType.Raw}
                    value={selectedType} 
                    onChange={setSelectedType} 
                    placeholder="Select generation type of stack with power based count">
                    <Select.Option value={AssetGenerationType.Static}>Static(stack doesn't grow over time)</Select.Option>
                    <Select.Option value={AssetGenerationType.Dynamic}>Dynamic(stack grows over time)</Select.Option>
                </Select>
            </Space>
        </Modal>
    </>
}

export default HeroAssetStackCreator;