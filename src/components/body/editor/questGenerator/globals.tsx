import { Button, Stack } from "@mantine/core";
import { useCurrentQuestId } from "./store";
import { invoke } from "@tauri-apps/api/core";

function QuestGeneratorGlobals() {
    const currentId = useCurrentQuestId();

    async function addToQueue() {
        await invoke("add_quest_to_queue", {questId: currentId!})
    }

    return (
    <Stack>
        <Button 
            disabled={currentId == undefined} 
            radius={0} 
            size="xs"
            onClick={addToQueue}
        >Добавить квест в очередь</Button>
        <Button
            radius={0}
            size="xs"
            onClick={() => invoke("apply_modifications")}
        >Применить изменения к карте
        </Button>
    </Stack>
    )
}

export default QuestGeneratorGlobals;