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

    return (
    <Button bg="green" c="dark" h={75} style={{position: 'absolute', right: 20, bottom: 10}} onClick={() => generateCreature()} radius={0}>
        <div>
            <span>Сгенерировать файлы существа</span>
            <br/>
            <span>{`${currentCreature?.name} [${currentCreature?.id}]`}</span>
        </div>
    </Button> )
}

export default CreatureDataGenerator;