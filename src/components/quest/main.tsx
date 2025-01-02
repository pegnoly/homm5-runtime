import QuestData from "./base_data/main";
import QuestProgressMain from "./progress/main";
import { QuestEditionState, useQuestStateStore } from "../../stores/QuestStateStore";
import { Segmented, Typography } from "antd";
import QuestInitializator from "./initialize";
import { useCurrentQuestStore } from "../../stores/QuestStore";
import { useShallow } from "zustand/shallow";
import QuestTextEditor from "./texts/QuestTextEditor";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

function QuestMain() {

    const setQuestEditorState = useQuestStateStore((state) => state.set_edition_state)
    const currentQuestId = useCurrentQuestStore((state) => state.id)

    return <>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-between', paddingBottom: '1%'}}>
            <CurrentQuest/>
            <Segmented style={{height: '50%'}}
                onChange={setQuestEditorState}
                options={[
                    {value: QuestEditionState.NotSelected, label: "Create/load quest"},
                    {value: QuestEditionState.BaseDataEdit, label: "Base data", disabled: currentQuestId == null},
                    {value: QuestEditionState.ProgressEdit, label: "Progresses", disabled: currentQuestId == null},
                    {value: QuestEditionState.TextEdit, label: "Texts", disabled: currentQuestId == null}
                ]}
            />
        </div>
        <div>
            <QuestStateRenderer/>
        </div>
    </>
}

function CurrentQuest() {

    const [questId, questName, setQuestName] = useCurrentQuestStore(useShallow((state) => [state.id, state.name, state.set_name]))

    useEffect(() => {
        if (questId != null) {
            loadName()
        }
    }, [questId])

    const loadName = async () => {
        await invoke<string>("load_quest_name", {questId: questId}).then((res) => setQuestName(res))
    }

    function getQuestText() {
        return questId == null ? "Not selected" : questName
    }

    return <div style={{display: 'flex', flexDirection: 'row', gap: 5}}>
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 19, fontWeight: 'bold'}}>Current quest:</Typography.Text>
        <Typography.Text style={{color: questId == null ? "red" : "green", fontFamily: 'cursive', fontSize: 19, fontWeight: 'bold'}}>{getQuestText()}</Typography.Text>
    </div>
}

function QuestStateRenderer () {
    
    const questEditorState = useQuestStateStore((state) => state.edition_state);

    switch (questEditorState) {
        case QuestEditionState.NotSelected:
            return <QuestInitializator/>
            break        
        case QuestEditionState.BaseDataEdit:
            return <QuestData/>
            break
        case QuestEditionState.ProgressEdit:
            return <QuestProgressMain/>
            break
        case QuestEditionState.TextEdit:
            return <QuestTextEditor/>
            break
    }
}

export default QuestMain;