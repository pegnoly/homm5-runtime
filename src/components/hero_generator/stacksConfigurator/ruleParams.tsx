import { Button, List, Select, Space, Typography } from "antd";
import { generationParamsNames, StackGenerationParam, StackGenerationRules } from "./main";
import { DeleteFilled, PlusOutlined } from "@ant-design/icons";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function HeroAssetStackGenerationRuleParams(params: {
    stackId: number,
    rules: StackGenerationRules,
    updateCallback: (value: StackGenerationRules) => void
}) {

    async function addRule(value: StackGenerationParam) {
        params.updateCallback({...params.rules, params: [...params.rules.params, value]});
        await invoke("add_stack_generation_rule", {stackId: params.stackId, rule: value});
    }

    async function removeRule(value: StackGenerationParam) {
        let updatedRules = params.rules.params.filter(param => param != value);
        params.updateCallback({...params.rules, params: updatedRules});
        await invoke("remove_stack_generation_rule", {stackId: params.stackId, rule: value});
    }

    return <div style={{display: 'flex', flexDirection: 'column'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stack generation rules</Typography.Text>
        <RuleSelector existing={params.rules.params} selectedCallback={addRule}/>
        <RulesList current={params.rules.params} removeCallback={removeRule}/>
    </div>
}

function RuleSelector(params: {
    existing: StackGenerationParam[],
    selectedCallback: (value: StackGenerationParam) => void
}) {

    const [selectedRule, setSelectedRule] = useState<StackGenerationParam | null>(null);

    return <Space>
        <Select
            style={{width: 150}}
            value={selectedRule}
            onChange={setSelectedRule}
        >{Array.from(generationParamsNames.entries()).map((value, index) => (
            <Select.Option key={index} value={value[0]}>{value[1]}</Select.Option>
        ))}</Select>    
        <Button 
            icon={<PlusOutlined/>}
            disabled={!selectedRule}
            onClick={() => {
                params.selectedCallback(selectedRule!);
                setSelectedRule(null);
            }}
        />
    </Space>
}

function RulesList(params: {
    current: StackGenerationParam[],
    removeCallback: (value: StackGenerationParam) => void
}) {

    return <>
        <List>{params.current.map((rule, index) => (
            <List.Item key={index}>
                <Space>
                    <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bolder', fontSize: 15}}>{generationParamsNames.get(rule)}</Typography.Text>
                    <Button onClick={() => params.removeCallback(rule)} icon={<DeleteFilled/>}/>
                </Space>
            </List.Item>
        ))}</List>
    </>
}

export default HeroAssetStackGenerationRuleParams;