import { Button, Select } from "antd";
import { QuestLoadingData, useQuestStateStore } from "../../stores/QuestStateStore";
import { useShallow } from "zustand/shallow";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useCurrentQuestStore } from "../../stores/QuestStore";

function QuestLoader() {

    const [currentMapQuests, setCurrentMapQuests] = useQuestStateStore(useShallow((state) => [state.current_map_quests, state.set_current_map_quests]))
    const setCurrentQuestId = useCurrentQuestStore((state) => state.set_id)

    const [selectedId, setSelectedId] = useState<string | null>(null);

    useEffect(() => {
        invoke<QuestLoadingData[]>("collect_quests_for_selection").then((quests) => setCurrentMapQuests(quests))
    }, [])

    async function selectQuest() {
        setCurrentQuestId(selectedId!)
    }

    return <div style={{display: 'flex', flexDirection: 'column', width: '45%', gap: 2}}>
        <Select
            value={selectedId}
            onChange={(id) => setSelectedId(id)}
            style={{height: 25}}
        >{currentMapQuests.map((q, i) => (
            <Select.Option key={i} value={q.id}>{q.name}</Select.Option>
        ))}</Select>
        <Button
            onClick={selectQuest} 
            style={{height: 25}}
        >Load existing quest</Button>
    </div>
}

export default QuestLoader;