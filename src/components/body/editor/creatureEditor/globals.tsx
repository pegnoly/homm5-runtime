import {Button} from "@mantine/core";
import {invoke} from "@tauri-apps/api/core";
import {TimelineMessage} from "@/components/timeline/types.ts";
import {EditorTimelineStore} from "@/components/timeline/store.ts";

function CreatureEditorGlobals() {
    const actions = EditorTimelineStore.useActions();
    async function RegenerateGroups() {
        await invoke<TimelineMessage>("rebuild_creatures_shared_group")
            .then((value) => {
                actions.addItem(value);
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000)
            })
    }

    return (
        <Button
            onClick={() => RegenerateGroups()}
            size="xs"
            style={{width: '100%'}}
            radius={0}>Regenerate groups</Button>
    )
}

export default CreatureEditorGlobals;