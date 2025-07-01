import { invoke } from "@tauri-apps/api/core";
import { Text } from "@mantine/core";
import { DifficultyMappedValue, DifficultyType, gameDifficultyNames } from "../../types";
import EditableProperty, { EditablePropertyWrapper } from "../../../../../common/editableProperty";

function DifficultyValues(params: {
    name: string,
    containerId: number, 
    values: DifficultyMappedValue, 
    updateCallback: (newValues: DifficultyMappedValue) => void,
    tauriFunction: string,
    tooltipComponent?: EditablePropertyWrapper
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
        <Text style={{fontSize: 15, fontWeight: 'bold', color: 'darkorchid'}}>{params.name}</Text>
        <div style={{display: 'flex', flexDirection: 'column', gap: '5%'}}>
            <EditableProperty 
                label={gameDifficultyNames.get(DifficultyType.Easy)!} 
                initialValue={params.values.data[DifficultyType.Easy].toString()}
                onSave={updateEasyValue}
                tooltip={params.tooltipComponent != undefined ? {component: params.tooltipComponent?.component!} : undefined}
            />
            <EditableProperty 
                label={gameDifficultyNames.get(DifficultyType.Medium)!} 
                initialValue={params.values.data[DifficultyType.Medium].toString()} 
                onSave={updateMediumValue}
                tooltip={params.tooltipComponent != undefined ? {component: params.tooltipComponent?.component!} : undefined}
            />
            <EditableProperty 
                label={gameDifficultyNames.get(DifficultyType.Hard)!} 
                initialValue={params.values.data[DifficultyType.Hard].toString()} 
                onSave={updateHardValue}
                tooltip={params.tooltipComponent != undefined ? {component: params.tooltipComponent?.component!} : undefined}
            />
            <EditableProperty 
                label={gameDifficultyNames.get(DifficultyType.Heroic)!} 
                initialValue={params.values.data[DifficultyType.Heroic].toString()} 
                onSave={updateHeroicValue}
                tooltip={params.tooltipComponent != undefined ? {component: params.tooltipComponent?.component!} : undefined}
            />
        </div>
    </div>
}

export default DifficultyValues;