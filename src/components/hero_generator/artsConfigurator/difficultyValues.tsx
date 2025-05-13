import { invoke } from "@tauri-apps/api/core";
import { DifficultyMappedValue, DifficultyType, gameDifficultyNames } from "./types";
import { Space, Typography } from "antd";
import BankStringProperty from "../../bank_configurator/utils";

function DifficultyValues(params: {
    name: string,
    containerId: number, 
    values: DifficultyMappedValue, 
    updateCallback: (newValues: DifficultyMappedValue) => void,
    tauriFunction: string
}) {

    async function updateEasyValue(newValue: string) {
        await invoke<number>(params.tauriFunction, {containerId: params.containerId, difficulty: DifficultyType.Easy, value: newValue})
            .then((value) => {
                params.values.data[DifficultyType.Easy] = value;
                params.updateCallback(params.values);
            });
    }

    async function updateMediumValue(newValue: string) {
        await invoke<number>(params.tauriFunction, {containerId: params.containerId, difficulty: DifficultyType.Medium, value: newValue})
            .then((value) => {
                params.values.data[DifficultyType.Medium] = value;
                params.updateCallback(params.values);
            });
    }

    async function updateHardValue(newValue: string) {
        await invoke<number>(params.tauriFunction, {containerId: params.containerId, difficulty: DifficultyType.Hard, value: newValue})
            .then((value) => {
                params.values.data[DifficultyType.Hard] = value;
                params.updateCallback(params.values);
            });
    }

    async function updateHeroicValue(newValue: string) {
        await invoke<number>(params.tauriFunction, {containerId: params.containerId, difficulty: DifficultyType.Heroic, value: newValue})
            .then((value) => {
                params.values.data[DifficultyType.Heroic] = value;
                params.updateCallback(params.values);
            });
    }

    return <div style={{display: 'flex', flexDirection: 'column'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 20, color: 'darkorchid'}}>{params.name}</Typography.Text>
        <Space direction="vertical">
            <BankStringProperty 
                text={gameDifficultyNames.get(DifficultyType.Easy)!} 
                initialValue={params.values.data[DifficultyType.Easy]} 
                updateCallback={updateEasyValue}
            />
            <BankStringProperty 
                text={gameDifficultyNames.get(DifficultyType.Medium)!} 
                initialValue={params.values.data[DifficultyType.Medium]} 
                updateCallback={updateMediumValue}
            />
            <BankStringProperty 
                text={gameDifficultyNames.get(DifficultyType.Hard)!} 
                initialValue={params.values.data[DifficultyType.Hard]} 
                updateCallback={updateHardValue}
            />
            <BankStringProperty 
                text={gameDifficultyNames.get(DifficultyType.Heroic)!} 
                initialValue={params.values.data[DifficultyType.Heroic]} 
                updateCallback={updateHeroicValue}
            />
        </Space>
    </div>
}

export default DifficultyValues;