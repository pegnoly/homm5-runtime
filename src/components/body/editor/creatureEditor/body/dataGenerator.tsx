import {Button} from "@mantine/core";
import {invoke} from "@tauri-apps/api/core";
import {CreatureEditorStore} from "@/components/body/editor/creatureEditor/store.ts";
import {EditorTimelineStore} from "@/components/timeline/store.ts";
import {TimelineMessage} from "@/components/timeline/types.ts";

function CreatureDataGenerator() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = EditorTimelineStore.useActions();

    async function generateCreature() {
        await invoke<TimelineMessage>("generate_creature_file", {id: currentCreature?.id})
            .then((value) => {
                actions.addItem(value);
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000)
            });
    }

    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-around', alignItems: 'center'}}>
            <Button
                radius={0}
                disabled={currentCreature == undefined}
                onClick={() => generateCreature()}
            >Generate game files for creature</Button>
    </div>
}

export default CreatureDataGenerator;