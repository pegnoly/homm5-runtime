import EditableProperty from "@/components/common/editableProperty";
import { CreatureEditorStore } from "../store"
import { Checkbox, Stack, Text } from "@mantine/core";
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

    async function updateIsGeneratable(value: boolean) {
        await invoke(`update_creature_is_generatable`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "is_generatable", value)
                actions.updateCreature(newModel);
            })      
    }

    async function updateIsFlying(value: boolean) {
        await invoke(`update_creature_is_flying`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "is_flying", value)
                actions.updateCreature(newModel);
            })      
    }

    async function updateIsUpgrade(value: boolean) {
        await invoke(`update_creature_is_upgrade`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "is_upgrade", value)
                actions.updateCreature(newModel);
            })      
    }

    return (
    <>
    {
        currentCreature == undefined ? null :
        <>
            <div style={{width: '25%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '0.75%', paddingTop: '1%'}}>
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
            }
                <Stack gap="xs">
                    <Checkbox
                        radius={0}
                        label="Is generatable"
                        checked={currentCreature.is_generatable}
                        onChange={(e) => updateIsGeneratable(e.currentTarget.checked)}
                    />
                    <Checkbox
                        radius={0}
                        label="Is flying"
                        checked={currentCreature.is_flying}
                        onChange={(e) => updateIsFlying(e.currentTarget.checked)}
                    />
                    <Checkbox
                        radius={0}
                        label="Is upgrade"
                        checked={currentCreature.is_upgrade}
                        onChange={(e) => updateIsUpgrade(e.currentTarget.checked)}
                    />
                </Stack>
            </div>
        </>
    }
    </>
    )
}

export default CreatureNumberPropsEditor;