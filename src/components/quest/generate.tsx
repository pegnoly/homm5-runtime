import { Button } from "antd";
import { useCurrentQuestStore } from "../../stores/QuestStore";
import { invoke } from "@tauri-apps/api/core";

function QuestGenerator() {
    const questId = useCurrentQuestStore(state => state.id);

    async function addQuestToQueue() {
        await invoke("add_quest_to_queue", {questId: questId})
    }

    async function applyModifications() {
        await invoke("apply_modifications")
    }

    return <div style={{display: 'flex', alignItems: 'center', flexDirection: 'column', gap: 8}}>
        <Button size="large" onClick={addQuestToQueue} disabled={questId == null}>Add quest to queue</Button>
        <Button size="large" onClick={applyModifications}>Apply quests to map</Button>
    </div>
}

export default QuestGenerator;