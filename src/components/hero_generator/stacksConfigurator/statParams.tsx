import { PlusOutlined } from "@ant-design/icons"
import { invoke } from "@tauri-apps/api/core"
import { Button, List, Modal, Select, Space, Typography } from "antd"
import { useEffect, useState } from "react"


function HeroAssetStackStatGenerator(params: {
    stackId: number
}) {
    const [elements, setElements] = useState<ArmyGenerationStatElement[]>([]);

    useEffect(() => {
        loadElements();
    }, [params.stackId])

    const loadElements = async () => {
        await invoke<ArmyGenerationStatElement[]>("load_stats_generation_elements", {stackId: params.stackId})
            .then((values) => {
                console.log("Values: ", values);
                setElements(values);
            });
    }

    async function onElementCreated(value: ArmyGenerationStatElement) {
        setElements([...elements, value]);
    }

    async function onElementUpdated(value: ArmyGenerationStatElement) {
        console.log("Got updated element: ", value);
        const updatedElements = elements.map((element) => {
            if (element.id == value.id) {
                return value;
            } else {
                return element;
            }
        })
        console.log("Elements after update: ", updatedElements);
        setElements(updatedElements);
    }

    return <Space direction="vertical">
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stats generation rules</Typography.Text>
        <HeroAssetStatElementCreator elementsCount={elements.length} stackId={params.stackId} elementCreatedCallback={onElementCreated}/>
        <List>{elements.map((element, index) => (
            <List.Item key={index}>
                <HeroAssetStatElementItem element={element} updateCallback={onElementUpdated}/>
            </List.Item>
        ))}</List>
    </Space>
}

function HeroAssetStatElementCreator(params: {
    stackId: number
    elementsCount: number
    elementCreatedCallback: (created: ArmyGenerationStatElement) => void
}) {
    const [rule, setRule] = useState<StatGenerationRule | null>(null);
    const [opened, setOpened] = useState<boolean>(false);

    async function close() {
        setOpened(false);
    }

    async function create() {
        await close();
        await invoke<ArmyGenerationStatElement>("add_stat_generation_element", {stackId: params.stackId, rule: rule})
            .then((value) => params.elementCreatedCallback(value));
    }

    return <>
        <Button 
            disabled={params.elementsCount >= 1}
            onClick={() => setOpened(true)} icon={<PlusOutlined/>}
        />
        <Modal
            open={opened}
            onCancel={close}
            onClose={close}
            onOk={create}
        >
            <Select 
                value={rule}
                onChange={setRule}
                placeholder="Select stat generation rule"
            >
                <Select.Option key={1} value={StatGenerationRule.MaxBy}>Max by</Select.Option>
                <Select.Option key={2} value={StatGenerationRule.MinBy}>Min by</Select.Option>
            </Select>
        </Modal>
    </>
}

function HeroAssetStatElementItem(params: {
    element: ArmyGenerationStatElement,
    updateCallback: (value: ArmyGenerationStatElement) => void
}) {

    async function updateParams(values: StatGenerationType[]) {
        await invoke("update_stat_generation_params", {elementId: params.element.id, params: values});
        params.updateCallback({...params.element, stats: {...params.element.stats, values: values}});
    }

    async function updatePriority(value: string) {
        await invoke("update_stat_generation_element_priority", {elementId: params.element.id, priority: parseInt(value)});
        params.updateCallback({...params.element, priority: parseInt(value)});
    }

    async function updateRule(value: StatGenerationRule) {
        await invoke("update_stat_generation_element_rule", {elementId: params.element.id, rule: value});
        params.updateCallback({...params.element, rule: value});
    }

    return <Space size="middle">
        <Select 
            style={{width: 200}}
            mode="multiple"
            value={params.element.stats.values}
            onChange={updateParams}
        >
            <Select.Option key={0} value={StatGenerationType.Attack}>Attack</Select.Option>
            <Select.Option key={1} value={StatGenerationType.Defence}>Defence</Select.Option>
            <Select.Option key={2} value={StatGenerationType.Initiative}>Initiative</Select.Option>
            <Select.Option key={3} value={StatGenerationType.Speed}>Speed</Select.Option>
            <Select.Option key={4} value={StatGenerationType.Hitpoints}>Hitpoints</Select.Option>
        </Select>
        <Typography.Text editable={{onChange: updatePriority}}>{params.element.priority}</Typography.Text>
        <Select
            value={params.element.rule}
            onChange={updateRule}
        >
            <Select.Option key={0} value={StatGenerationRule.MaxBy}>MaxBy</Select.Option>
            <Select.Option key={1} value={StatGenerationRule.MinBy}>MinBy</Select.Option>
        </Select>
    </Space>
}

export default HeroAssetStackStatGenerator;