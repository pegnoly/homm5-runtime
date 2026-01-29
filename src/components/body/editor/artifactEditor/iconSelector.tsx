import { listen } from "@tauri-apps/api/event";
import { ArtifactEditorStore } from "./store";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";
import { Button, Group, Text } from "@mantine/core";

function ArtifactIconSelector() {
    const current = ArtifactEditorStore.useCurrent();
    const action = ArtifactEditorStore.useActions();

    listen<string | null>("artifact_icon_path_selected", e => {
        console.log("Icon selected: ", e.payload);
        console.log("Current", current?.id);
        invoke("update_artefact_icon_path", {id: current?.id, value: `${e.payload?.toLowerCase().replace("\\", "/")}/icon.xdb`, path: `${e.payload}/icon.xdb`})
            .then(() => {
                var updated = ObjectUtils.updateObjectDynamically(current!, "icon_xdb", `${e.payload?.toLowerCase().replace("\\", "/")}/icon.xdb`);
                action.updateCurrent(updated);
            });
    })

    return <>
    {
        current == undefined ? null :
        <Group maw={550}>
            <Text>{current.icon_xdb}</Text>
            <Button 
                radius={0}
                onClick={() => {
                    invoke("select_artefact_icon_path")
                }}
            >Select icon path</Button>
        </Group>
    }
    </>
}

export default ArtifactIconSelector;