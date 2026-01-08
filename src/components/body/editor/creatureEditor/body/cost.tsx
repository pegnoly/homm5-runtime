import EditableProperty from "@/components/common/editableProperty";
import { CreatureEditorStore } from "../store";
import { ObjectUtils } from "@/lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { Text } from "@mantine/core";

function CreatureCostEditor() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();

    async function updateCost(param: string, value: number) {
        const updatedCost = ObjectUtils.updateObjectDynamically(currentCreature?.cost!, param, value);
        await invoke("update_creature_cost", {id: currentCreature?.id, value: updatedCost})
            .then(() => {
                const updatedModel = ObjectUtils.updateObjectDynamically(currentCreature!, "cost", updatedCost);
                actions.updateCreature(updatedModel);
            });
    }

    return (
    <>
    {
        currentCreature == undefined ? null :
        <div style={{width: '100%', display: 'flex', flexDirection: 'column', gap: '5%', alignContent: 'center'}}>
            <Text style={{fontSize: 20}}>Cost information</Text>
            <div style={{width: '100%', display: 'flex', flexDirection: 'row', gap: '5%'}}>
                <div style={{width: '45%', display: 'flex', flexDirection: 'column', gap: '3%'}}>
                    <EditableProperty 
                        initialValue={currentCreature.cost.gold}
                        label="Gold"
                        onSave={(value) => updateCost("gold", value as number)}
                    />
                    <EditableProperty 
                        initialValue={currentCreature.cost.wood}
                        label="Wood"
                        onSave={(value) => updateCost("wood", value as number)}
                    />
                    <EditableProperty 
                        initialValue={currentCreature.cost.ore}
                        label="Ore"
                        onSave={(value) => updateCost("ore", value as number)}
                    />
                    <EditableProperty 
                        initialValue={currentCreature.cost.gem}
                        label="Gems"
                        onSave={(value) => updateCost("gem", value as number)}
                    />
                </div>
                <div style={{width: '45%', display: 'flex', flexDirection: 'column', gap: '3%'}}>
                    <EditableProperty 
                        initialValue={currentCreature.cost.crystal}
                        label="Crystal"
                        onSave={(value) => updateCost("crystal", value as number)}
                    />
                    <EditableProperty 
                        initialValue={currentCreature.cost.sulfur}
                        label="Sulfur"
                        onSave={(value) => updateCost("sulfur", value as number)}
                    />
                    <EditableProperty 
                        initialValue={currentCreature.cost.mercury}
                        label="Mercury"
                        onSave={(value) => updateCost("mercury", value as number)}
                    />
                </div>
            </div>
        </div>
    }
    </>
    )
}

export default CreatureCostEditor;