import { Col, Row, Select, Space, Typography } from "antd";
import { HeroAssetStackModel, StackGenerationRules, TownType } from "./main";
import HeroAssetStackGenerationRuleParams from "./ruleParams";
import { invoke } from "@tauri-apps/api/core";

function HeroAssetStackGenerationRules(params: {
    model: HeroAssetStackModel, 
    updateCallback: (value: HeroAssetStackModel) => void
}) {
    async function updateTown(value: TownType) {
        params.updateCallback({...params.model, town: value});
        await invoke("update_stack_creature_town", {stackId: params.model.id, town: value});
    }

    async function updateTier(value: number) {
        params.updateCallback({...params.model, tier: value});
        await invoke("update_stack_creature_tier", {stackId: params.model.id, tier: value});
    }

    async function updateGenerationRules(value: StackGenerationRules) {
        params.updateCallback({...params.model, generation_rule: value});
    }

    return <Row>
        <Col span={10}>
            <Space direction="vertical">
                <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stack unit base data</Typography.Text>
                <UnitTownSelector currentTown={params.model.town} updateCallback={updateTown}/>
                <UnitTierSelector currentTier={params.model.tier} updateCallback={updateTier}/>
            </Space>
        </Col>
        <Col span={12}>
            <Space direction="vertical">
                <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stack generation rules</Typography.Text>
                <HeroAssetStackGenerationRuleParams stackId={params.model.id} rules={params.model.generation_rule} updateCallback={updateGenerationRules}/>
            </Space>
        </Col>
    </Row>
}

function UnitTownSelector(params: {
    currentTown: TownType,
    updateCallback: (value: TownType) => void
}) {
    return <Space direction="vertical">
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 17, fontWeight: 'bold'}}>Unit town</Typography.Text>
        <Select 
            value={params.currentTown}
            onChange={params.updateCallback}
            style={{width: 150}}
            placeholder="Select town of creature"
        >
            <Select.Option key={1} value={TownType.TownHeaven}>Heaven</Select.Option>
            <Select.Option key={2} value={TownType.TownInferno}>Inferno</Select.Option>
            <Select.Option key={3} value={TownType.TownNecromancy}>Necromancy</Select.Option>
            <Select.Option key={4} value={TownType.TownPreserve}>Preserve</Select.Option>
            <Select.Option key={5} value={TownType.TownDungeon}>Dungeon</Select.Option>
            <Select.Option key={6} value={TownType.TownAcademy}>Academy</Select.Option>
            <Select.Option key={7} value={TownType.TownFortress}>Fortress</Select.Option>
            <Select.Option key={8} value={TownType.TownStronghold}>Stronghold</Select.Option>
        </Select>
    </Space>
}

function UnitTierSelector(params: {
    currentTier: number,
    updateCallback: (value: number) => void
}) {
    return <Space direction="vertical">
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 17, fontWeight: 'bold'}}>Unit tier</Typography.Text>
        <Select 
            value={params.currentTier}
            onChange={params.updateCallback}
            style={{width: 150}}
            placeholder="Select tier of creature"
        >
            <Select.Option key={1} value={1}>1</Select.Option>
            <Select.Option key={2} value={2}>2</Select.Option>
            <Select.Option key={3} value={3}>3</Select.Option>
            <Select.Option key={4} value={4}>4</Select.Option>
            <Select.Option key={5} value={5}>5</Select.Option>
            <Select.Option key={6} value={6}>6</Select.Option>
            <Select.Option key={7} value={7}>7</Select.Option>
        </Select>
    </Space>
}

export default HeroAssetStackGenerationRules;