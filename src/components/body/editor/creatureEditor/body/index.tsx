import { Text } from "@mantine/core";
import { CreatureEditorStore } from "../store";
import EditableProperty from "@/components/common/editableProperty";
import { CreatureEditableModel } from "../types";
import { invoke } from "@tauri-apps/api/core";

const creatureNumberProperties = [
    'attack', 
    'defence', 
    'min_damage', 
    'max_damage', 
    'speed', 
    'inititative',
    'health',
    'spell_points',
    'exp',
    'power',
    'tier',
    'grow',
    'size',
    'range',
    'shots'
]

function updateObjectDynamically<T extends object>(
    obj: T,
    path: string,
    value: any
): T {
    const keys = path.split('.');
    const result = { ...obj };
    
    let current: any = result;
    for (let i = 0; i < keys.length - 1; i++) {
        const key = keys[i];
        current[key] = { ...current[key] };
        current = current[key];
    }
    
    current[keys[keys.length - 1]] = value;
    return result;
}

function getFieldValue<T, K extends keyof T>(obj: T, key: K): T[K] {
    return obj[key];
}


function CreatureEditorBody() {
    
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();

    async function updateCreatureNumberParam(paramName: string, value: number) {
        await invoke(`update_creature_${paramName}`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, paramName, value)
                actions.updateCreature(newModel);
            })
    }

    return (
    <>
        {
            currentCreature == undefined ? null :
            <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div style={{width: '25%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1.5%', paddingTop: '2%'}}>
                <Text style={{textAlign: 'center', fontSize: 20}}>Numeric params</Text>
                {
                    Object.entries(currentCreature).filter(v => creatureNumberProperties.includes(v[0])).map(v => (
                        <EditableProperty
                            key={v[0]}
                            initialValue={getFieldValue(currentCreature, v[0] as keyof CreatureEditableModel) as number}
                            onSave={(value) => updateCreatureNumberParam(v[0], value as number)}
                            label={v[0]}
                        />
                    ))
                }</div>
            </div>
        }
    </>
    )
}

export default CreatureEditorBody;