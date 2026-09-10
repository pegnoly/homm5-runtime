import { CreatureEditorStore } from "../store";
import { Button, Stack, Textarea, TextInput } from "@mantine/core";
import { useEffect, useState } from "react";
import useGameDataStore from "@/stores/GameDataStore";
import { useShallow } from "zustand/shallow";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";

function CreatureTextsEditor() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();
    const [creatures, updateCreatures] = useGameDataStore(useShallow(state => [state.creatures, state.load_creatures]));

    const [localDesc, setLocalDesc] = useState<string | undefined>(currentCreature == undefined ? undefined : currentCreature.desc);
    const [localName, setLocalName] = useState<string | undefined>(currentCreature == undefined ? undefined : currentCreature.name);
    const [descEditable, setDescEditable] = useState<boolean>(false);
    const [nameEditable, setNameEditable] = useState<boolean>(false);

    useEffect(() => {
        if (currentCreature != undefined) {
            setLocalDesc(currentCreature.desc);
            setLocalName(currentCreature.name);
        }
    }, [currentCreature])

    async function saveDesc() {
        await invoke(`update_creature_desc`, {id: currentCreature?.id, value: localDesc})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "desc", localDesc)
                actions.updateCreature(newModel);
            })   
    }

    async function saveName() {
        await invoke(`update_creature_name`, {id: currentCreature?.id, value: localName})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "name", localName)
                actions.updateCreature(newModel);
                const updatedCreatures = creatures.map(cr => {
                    if (cr.id == currentCreature?.id) {
                        cr.name = localName!;
                        return cr;
                    } else {
                        return cr;
                    }
                });
                updateCreatures(updatedCreatures);
            })   
    }

    return (
    <>
        <Stack>
            <div>
                <Button
                    onClick={() => {
                        if (descEditable) {
                            setDescEditable(false);
                            saveDesc()
                        } else {
                            setDescEditable(true);
                        }
                    }}
                    size="xs"
                    radius={0}
                >{!descEditable ? "Редактировать описание" : "Сохранить"}</Button>
                <Textarea
                    value={localDesc}
                    onChange={(e) => setLocalDesc(e.currentTarget.value)}
                    rows={12}
                    disabled={!descEditable}
                />
            </div>
            <div>
                <Button
                    onClick={() => {
                        if (nameEditable) {
                            setNameEditable(false);
                            saveName()
                        } else {
                            setNameEditable(true)
                        }
                    }}
                    size="xs"
                    radius={0}
                >{!nameEditable ? "Редактировать имя" : "Сохранить"}</Button>
                <TextInput
                    value={localName}
                    onChange={(e) => setLocalName(e.currentTarget.value)}
                    disabled={!nameEditable}
                />
            </div>
        </Stack>
    </>
    )
}

export default CreatureTextsEditor;