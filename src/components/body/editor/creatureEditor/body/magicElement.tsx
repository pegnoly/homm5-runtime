import { Select, Text } from "@mantine/core";
import { CreatureEditorStore } from "../store";
import { MagicElement } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";

function MagicElementUpdater() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();

    async function updateFirstMagicElement(value: MagicElement) {
        const updatedElement = { first: value, second: currentCreature?.magic_element.second };
        await invoke("update_creature_magic_element", {id: currentCreature?.id, value: updatedElement})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "magic_element", updatedElement)
                actions.updateCreature(newModel);
            });
    }

    async function updateSecondMagicElement(value: MagicElement) {
        const updatedElement = { first: currentCreature?.magic_element.first, second: value };
        await invoke("update_creature_magic_element", {id: currentCreature?.id, value: updatedElement})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "magic_element", updatedElement)
                actions.updateCreature(newModel);
            });
    }

    return (
    <>
        {
            currentCreature == undefined ? null :
            <>
                <Text style={{textAlign: 'center', fontSize: 20}}>Magic elements</Text>
                <div style={{width: '100%', display: 'flex', flexDirection: 'row', gap: '5%'}}>
                    <Select
                        style={{width: '45%'}}
                        radius={0}
                        label="First"
                        value={currentCreature.magic_element.first}
                        data={[
                            {label: 'None', value: MagicElement.None},
                            {label: 'Air', value: MagicElement.Air},
                            {label: 'Fire', value: MagicElement.Fire},
                            {label: 'Water', value: MagicElement.Water},
                            {label: 'Earth', value: MagicElement.Earth},
                        ]}
                        onChange={(value) => updateFirstMagicElement(value as MagicElement)}
                    />
                    <Select
                        style={{width: '45%'}}
                        radius={0}
                        label="Second"
                        value={currentCreature.magic_element.second}
                        data={[
                            {label: 'None', value: MagicElement.None},
                            {label: 'Air', value: MagicElement.Air},
                            {label: 'Fire', value: MagicElement.Fire},
                            {label: 'Water', value: MagicElement.Water},
                            {label: 'Earth', value: MagicElement.Earth},
                        ]}
                        onChange={(value) => updateSecondMagicElement(value as MagicElement)}
                    />
                </div>
            </>
        }
    </>
    )
}

export default MagicElementUpdater;