import EditableProperty from "@/components/common/editableProperty";
import { CreatureEditorStore } from "../store"
import { Text } from "@mantine/core";
import { CreatureEditableModel } from "../types";
import { ObjectUtils } from "@/lib/utils";
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

function CreatureNumberPropsEditor() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions()

    async function updateNumberParam(paramName: string, value: number) {
        await invoke(`update_creature_${paramName}`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, paramName, value)
                actions.updateCreature(newModel);
            })
    }


    return (
    <>
    {
        currentCreature == undefined ? null :
        <>
            <div style={{width: '25%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1.5%', paddingTop: '2%'}}>
            <Text style={{textAlign: 'center', fontSize: 20}}>Numeric params</Text>
            {
                Object.entries(currentCreature).filter(v => creatureNumberProperties.includes(v[0])).map(v => (
                    <EditableProperty
                        key={v[0]}
                        initialValue={ObjectUtils.getFieldValue(currentCreature, v[0] as keyof CreatureEditableModel) as number}
                        onSave={(value) => updateNumberParam(v[0], value as number)}
                        label={v[0]}
                    />
                ))
            }</div>
        </>
    }
    </>
    )
}

export default CreatureNumberPropsEditor;