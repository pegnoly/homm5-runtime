import { useEffect, useState } from "react";
import { useCurrentQuestStore } from "../../../stores/QuestStore";
import { invoke } from "@tauri-apps/api/core";
import TextArea from "antd/es/input/TextArea";
import { Button, Checkbox, Typography } from "antd";
import { useShallow } from "zustand/shallow";

type ProgressData = {
    text: string,
    concatenate: boolean
}

function QuestProgressMain() {

    const [questId, currentProgress, setCurrentProgress] = useCurrentQuestStore(useShallow((state) => [state.id, state.current_progress, state.set_current_progress]));
    const [progressText, setProgressText] = useState<string>("");
    const [concatenate, setConcatenate] = useState<boolean>(true);

    useEffect(() => {
        console.log("Loading progress ", currentProgress)
        invoke<ProgressData>("load_progress", {questId: questId, number: 0})
            .then((pd) => {
                setProgressText(pd.text)
                setConcatenate(pd.concatenate)
            })
    }, [])

    async function saveProgress() {
        await invoke("save_progress", {questId: questId, number: currentProgress, text: progressText})
    }

    async function changeProgress(change: number) {
        setCurrentProgress(currentProgress + change);
        await invoke<ProgressData>("load_progress", {questId: questId, number: currentProgress + change})
            .then((pd) => {
                setProgressText(pd.text)
                setConcatenate(pd.concatenate)
            })
    }

    async function updateConcatenation(checked: boolean) {
        setConcatenate(checked)
        await invoke("update_progress_concatenation", {questId: questId, number: currentProgress, concatenate: checked})
    }

    return <>
        <TextArea 
            rows={20}
            value={progressText}
            onChange={(e) => setProgressText(e.currentTarget.value)}
        />
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', gap: 10, paddingTop: 15}}>
            <Button 
                disabled={currentProgress == 0}
                onClick={() => changeProgress(-1)}
            >Previous</Button>
            <Typography.Text style={{position: 'relative', bottom: -3}}>{currentProgress}</Typography.Text>
            <Button
                onClick={() => changeProgress(1)}
            >Next</Button>
            <Button 
                onClick={saveProgress}
            >Save current</Button>
        </div>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', paddingTop: '2%'}}>
            <Checkbox checked={concatenate} onChange={(e) => updateConcatenation(e.target.checked)}>Concatenate with previous</Checkbox>
        </div>
    </>
}

export default QuestProgressMain;
