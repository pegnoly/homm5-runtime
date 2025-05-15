import { Col, Row, Select, Space, Typography } from "antd";
import { HeroAssetStackModel, StackGenerationRules, StackUnitGenerationType, TownType } from "./main";
import { invoke } from "@tauri-apps/api/core";
import HeroAssetStackGenerationRuleParams from "./ruleParams";
import { useState } from "react";
import useGameDataStore from "../../../stores/GameDataStore";

function HeroAssetStackUnitConfigurator(params: {
    model: HeroAssetStackModel
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

    async function updateConcreteCreature(value: number) {
        params.updateCallback({...params.model, concrete_creature: value});
        await invoke("update_stack_concrete_creature", {stackId: params.model.id, creature: value});
    }

    return <>
    {
        params.model.type_generation_mode == StackUnitGenerationType.TierSlotBased ?
        <>
            <Row>
                <Col span={10}>
                    <TierSlotBasedSelector
                        assetId={params.model.id}
                        tier={params.model.tier}
                        town={params.model.town}
                        updateTierCallback={updateTier}
                        updateTownCallback={updateTown}
                    />
                </Col>
                <Col span={12}>
                    <HeroAssetStackGenerationRuleParams 
                        stackId={params.model.id} 
                        rules={params.model.generation_rule} 
                        updateCallback={updateGenerationRules}
                    />
                </Col>
            </Row>
        </> :
        <ConcreteUnitSelector 
            assetId={params.model.id}
            currentUnit={params.model.concrete_creature}
            unitSelectedCallback={updateConcreteCreature}
        />
    }
    </>
}

function ConcreteUnitSelector(params: {
    assetId: number,
    currentUnit: number,
    unitSelectedCallback: (value: number) => void    
}) {

    const [town, setTown] = useState<TownType>(TownType.TownNoType);
    const [tier, setTier] = useState<number>(1);
    const creatures = useGameDataStore((state) => state.creatures);

    return <Space direction="vertical" size="middle">
        <Typography.Text 
            style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}
        >Concrete creature selector</Typography.Text>
        <div>
            <Space>
                <Typography.Text style={{fontFamily: 'cursive', fontSize: 17, fontWeight: 'bold'}}>Unit town</Typography.Text>
                <Select 
                    value={town}
                    onChange={setTown}
                    style={{width: 150}}
                    placeholder="Select town of creature"
                >
                    <Select.Option key={0} value={TownType.TownNoType}>Neutral</Select.Option>
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
        </div>
        <div>
            <Space>
                <Typography.Text style={{fontFamily: 'cursive', fontSize: 17, fontWeight: 'bold'}}>Unit tier</Typography.Text>
                <Select 
                    value={tier}
                    onChange={setTier}
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
        </div>
        <div>
            <Space>
                <Typography.Text style={{fontFamily: 'cursive', fontSize: 17, fontWeight: 'bold'}}>Concrete units</Typography.Text>
                <Select 
                    showSearch
                    value={params.currentUnit}
                    onChange={params.unitSelectedCallback}
                    style={{width: 150}}
                    placeholder="Select concrete creature"
                    filterOption={(input, option) =>
                        (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
                    }
                    options={
                        creatures
                            .filter(creature => creature.tier == tier && creature.town == town)
                            .map((creature, _index) => ({value: creature.id, label: creature.name}))
                    }
                />
            </Space>
        </div>
    </Space>
}

function TierSlotBasedSelector(params: {
    assetId: number,
    town: TownType,
    tier: number,
    updateTownCallback: (value: TownType) => void,
    updateTierCallback: (value: number) => void 
}) {

    return <Space direction="vertical">
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stack unit base data</Typography.Text>
        <UnitTownSelector currentTown={params.town} updateCallback={params.updateTownCallback}/>
        <UnitTierSelector currentTier={params.tier} updateCallback={params.updateTierCallback}/>
    </Space>
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

export default HeroAssetStackUnitConfigurator;