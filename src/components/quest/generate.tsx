import { Button } from "antd";
import { useQuestStore } from "../../stores/QuestStore";
import { invoke } from "@tauri-apps/api/core";

function QuestGenerator() {
    const questId = useQuestStore(state => state.id);

    async function generateQuest() {
        await invoke("generate_quest", {questId: questId})
    }

    return <div style={{display: 'flex', height: 20, justifyContent: 'right', position: 'relative', bottom: 30, left: 240, width: 500}}>
        <Button size="large" onClick={generateQuest}>Сгенерировать квест</Button>
    </div>
}

export default QuestGenerator;