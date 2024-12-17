import { useShallow } from "zustand/shallow";
import { useQuestStore } from "../../stores/QuestStore";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Typography } from "antd";
import TextArea from "antd/es/input/TextArea";
import { EditOutlined } from "@ant-design/icons";

function QuestDesc() {
    const [id, desc, setDesc] = useQuestStore(useShallow((state) => [state.id, state.desc, state.set_desc]))
    const [editable, setEditable] = useState<boolean>(false);

    async function tryEditDesc() {
        if (editable == true) {
            setEditable(false)
            await invoke("update_quest_desc", {questId: id, desc: desc})
        }
        else {
            setEditable(true)
        }
    }

    return <div style={{display: 'flex', paddingTop: '3%', flexDirection: 'column', gap: 2}}>
        <Typography.Text 
            style={{fontFamily: 'fantasy'}}
        >Описание квеста:</Typography.Text>
        <TextArea 
            disabled={!editable}
            style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 13}} 
            onChange={(e) => setDesc(e.currentTarget.value)} 
            value={desc} rows={10}
        />
        <Button 
            onClick={tryEditDesc} 
            style={{position: 'relative', 'left': '20%', height: 25, width: '60%'}} 
            icon={<EditOutlined/>}
        >Редактировать описание</Button>
    </div>
}

export default QuestDesc;