import { Button } from "antd";
import { useQuestStore } from "../../stores/QuestStore";
import { invoke } from "@tauri-apps/api/core";

function QuestGenerator() {
    const questId = useQuestStore(state => state.id);

    async function addQuestToQueue() {
        await invoke("add_quest_to_queue", {questId: questId})
    }

    async function applyModifications() {
        await invoke("apply_modifications")
    }

    return <div style={{display: 'flex', justifyContent: 'right', bottom: 70, position: 'relative', left: 440, width: 400, gap: 10}}>
        <Button size="large" onClick={addQuestToQueue}>Добавить квест в очередь</Button>
        <Button size="large" onClick={applyModifications}>Применить к карте</Button>
    </div>
}

export default QuestGenerator;