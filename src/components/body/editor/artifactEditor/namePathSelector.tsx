import { listen } from "@tauri-apps/api/event";
import { ArtifactEditorStore } from "./store";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";
import { Button, Group, Text } from "@mantine/core";

function ArtifactNamePathSelector() {
    const current = ArtifactEditorStore.useCurrent();
    const action = ArtifactEditorStore.useActions();

    listen<string | null>("artifact_name_path_selected", e => {
        invoke("update_artefact_name_path", {id: current?.id, value: `${e.payload?.toLowerCase().replace("\\", "/")}/name.txt`});
        invoke("update_artefact_desc_path", {id: current?.id, value: `${e.payload?.toLowerCase().replace("\\", "/")}/desc.txt`})
            .then(() => {
                var updated = ObjectUtils.updateObjectDynamically(current!, "name_txt", `${e.payload?.toLowerCase().replace("\\", "/")}/name.txt`);
                updated = ObjectUtils.updateObjectDynamically(updated, "desc_txt", `${e.payload?.toLowerCase().replace("\\", "/")}/desc.txt`)
                action.updateCurrent(updated);
            });
    });

    return <>
    {
        current == undefined ? null :
        <Group maw={550}>
            <Text>{current.name_txt}</Text>
            <Button 
                radius={0}
                onClick={() => {
                    invoke("select_artefact_name_path")
                }}
            >Select icon path</Button>
        </Group>
    }
    </>
}

export default ArtifactNamePathSelector;