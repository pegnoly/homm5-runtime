import { listen } from "@tauri-apps/api/event";
import { ArtifactEditorStore } from "./store";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";
import { Button, Group, Text } from "@mantine/core";
import useGameDataStore from "@/stores/GameDataStore";
import { useEffect, useState } from "react";

function ArtifactPathsSelector() {
    const current = ArtifactEditorStore.useCurrent();
    const action = ArtifactEditorStore.useActions();
    const updateArtifacts = useGameDataStore(state => state.update_artifacts);

    const [namePath, setNamePath] = useState<string | undefined>(undefined);
    const [wasUpdatedFromEvent, setWasUpdatedFromEvent] = useState<boolean>(false);

    listen<string | null>("artifact_name_path_selected", e => {
        setNamePath(`${e.payload!.toLowerCase().replace("\\", "/")}/name.txt`);
        setWasUpdatedFromEvent(true);
    });

    useEffect(() => {
        if (current != undefined) {
            setNamePath(current.name_txt);
        }
    }, [current?.id]);

    useEffect(() => {
        if (namePath != undefined && wasUpdatedFromEvent == true) {
            setWasUpdatedFromEvent(false);
            invoke("update_artefact_texts_paths", {id: current?.id, value: namePath.replace("/name.txt", "")})
                .then(() => {
                    var updated = ObjectUtils.updateObjectDynamically(current!, "name_txt", namePath);
                    updated = ObjectUtils.updateObjectDynamically(updated, "desc_txt", namePath.replace("name.txt", "desc.txt"))
                    action.updateCurrent(updated);
                    updateArtifacts(updated);
                });
        }
    }, [namePath]);

    return <>
    {
        current == undefined ? null :
        <>

        <Group maw={550}>
            <Text>{namePath}</Text>
            <Button 
                radius={0}
                onClick={() => {
                    invoke("select_artefact_name_path")
                }}
            >Select name path</Button>
        </Group>
        </>
    }
    </>
}

export default ArtifactPathsSelector;