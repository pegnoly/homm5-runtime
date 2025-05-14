import { useEffect, useState } from "react"
import { AssetGenerationType, DifficultyMappedValue } from "../artsConfigurator/types"
import { Button, Modal, Segmented, Select, Space, Typography } from "antd"
import { PlusOutlined } from "@ant-design/icons"
import { invoke } from "@tauri-apps/api/core"
import HeroAssetFocusedStack from "./focused"

export enum TownType {
    TownNoType = "TOWN_NO_TYPE",
    TownHeaven = "TOWN_HEAVEN",
    TownInferno = "TOWN_INFERNO",
    TownNecromancy = "TOWN_NECROMANCY", 
    TownPreserve = "TOWN_PRESERVE",
    TownDungeon = "TOWN_DUNGEON",
    TownAcademy = "TOWN_ACADEMY",
    TownFortress = "TOWN_FORTRESS",
    TownStronghold = "TOWN_STRONGHOLD"
}

export enum StackGenerationParam {
    UpgradeOnly = "GENERATION_RULE_UPGRADE_ONLY",
    Generatable = "GENERATION_RULE_GENERATABLE",
    Shooter = "GENERATION_RULE_SHOOTER",
    Caster = "GENERATION_RULE_CASTER"
}

export const generationParamsNames = new Map<StackGenerationParam, string>([
    [StackGenerationParam.UpgradeOnly, "Upgrade only"],
    [StackGenerationParam.Generatable, "Generatable only"],
    [StackGenerationParam.Caster, "Casters only"],
    [StackGenerationParam.Shooter, "Shooters only"]
])

export type StackGenerationRules = {
    params: StackGenerationParam[]
}

export type HeroAssetStackModel = {
    id: number,
    generation_type: AssetGenerationType,
    base_powers: DifficultyMappedValue,
    powers_grow: DifficultyMappedValue | null,
    town: TownType,
    tier: number,
    generation_rule: StackGenerationRules
}

function HeroAssetStacksConfigurator(params: {assetId: number}) {
    const [stacksIds, setStacksIds] = useState<number[]>([]);
    const [selectedStack, setSelectedStack] = useState<number | null>(null);

    useEffect(() => {
        loadStacksIds();
    }, []);

    const loadStacksIds = async () => {
        await invoke<number[]>("load_stacks_ids", {assetId: params.assetId})
            .then((values) => setStacksIds(values));
    }

    async function stackCreated(stackId: number) {
        setStacksIds([...stacksIds, stackId]);
    }

    return <div style={{display: 'flex', flexDirection: 'column'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stacks data</Typography.Text>
        <Space>
            <Segmented
                value={selectedStack}
                onChange={setSelectedStack}
                options={stacksIds.map((id, index) => ({value: id, label: (index + 1).toString()}))}
            />
            <HeroAssetStackCreator disabled={stacksIds.length >= 7} assetId={params.assetId} createCallback={stackCreated}/>
        </Space>
        {
            selectedStack != null ?
            <HeroAssetFocusedStack stackId={selectedStack}/> :
            null
        }
    </div>
}

function HeroAssetStackCreator(params: {
    disabled: boolean,
    assetId: number, 
    createCallback: (createdId: number) => void
}) {
    const [opened, setOpened] = useState<boolean>(false);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(null);

    async function create() {
        await close();
        await invoke<number>("create_stack", {assetId: params.assetId, generationType: selectedType})
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
            centered
        >
            <Space direction="vertical">
                <Select value={selectedType} onChange={setSelectedType} placeholder="Select generation type of stack">
                    <Select.Option value={AssetGenerationType.Static}>Static generation</Select.Option>
                    <Select.Option value={AssetGenerationType.Dynamic}>Dynamic generation</Select.Option>
                </Select>
                <Button onClick={create}>Create</Button>
            </Space>
        </Modal>
    </>
}

export default HeroAssetStacksConfigurator;