import { useEffect, useState } from "react";
import { useQuestStore } from "../../../stores/QuestStore";
import { invoke } from "@tauri-apps/api/core";
import TextArea from "antd/es/input/TextArea";
import { Button, Checkbox, Typography } from "antd";

type ProgressData = {
    text: string,
    concatenate: boolean
}

function QuestProgressMain() {

    const questId = useQuestStore((state) => state.id);
    const [progress, setProgress] = useState<number>(0);
    const [progressText, setProgressText] = useState<string>("");
    const [concatenate, setConcatenate] = useState<boolean>(true);

    useEffect(() => {
        if (questId != "") {
            invoke("load_progress", {questId: questId, number: 0})
        }
    }, [questId])

    async function saveProgress() {
        await invoke("save_progress", {questId: questId, number: progress, text: progressText})
    }

    async function changeProgress(change: number) {
        setProgress(progress + change);
        await invoke<ProgressData>("load_progress", {questId: questId, number: progress + change})
            .then((pd) => {
                setProgressText(pd.text)
                setConcatenate(pd.concatenate)
            })
    }

    async function updateConcatenation(checked: boolean) {
        setConcatenate(checked)
        await invoke("update_progress_concatenation", {questId: questId, number: progress, concatenate: checked})
    }

    return <>
        <TextArea 
            rows={15}
            value={progressText}
            onChange={(e) => setProgressText(e.currentTarget.value)}
        />
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', gap: 10, paddingTop: 15}}>
            <Button 
                disabled={progress == 0}
                onClick={() => changeProgress(-1)}
            >Предыдущий</Button>
            <Typography.Text>{progress}</Typography.Text>
            <Button
                onClick={() => changeProgress(1)}
            >Следующий</Button>
            <Button 
                onClick={saveProgress}
            >Сохранить текущий прогресс</Button>
        </div>
        <Checkbox checked={concatenate} onChange={(e) => updateConcatenation(e.target.checked)}>Конкатенировать с предыдущим</Checkbox>
    </>
}

export default QuestProgressMain;
