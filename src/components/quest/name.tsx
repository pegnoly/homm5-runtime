import { useShallow } from "zustand/shallow";
import { useQuestStore } from "../../stores/QuestStore";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input, Typography } from "antd";
import { EditOutlined } from "@ant-design/icons";

function QuestName() {
    
    const [id, name, setName] = useQuestStore(useShallow((state) => [state.id, state.name, state.set_name]))
    const [editable, setEditable] = useState<boolean>(false);

    async function tryEditName() {
        if (editable == true) {
            setEditable(false)
            await invoke("update_quest_name", {questId: id, name: name})
        }
        else {
            setEditable(true)
        }
    }

    return <div style={{display: 'flex', paddingTop: '3%', flexDirection: 'row', gap: 3}}>
        <Typography.Text 
            style={{fontFamily: 'fantasy', position: 'relative', bottom: -3}}
        >Имя: </Typography.Text>
        <Input 
            style={{height: 30, width: '80%', fontFamily: 'cursive', fontWeight: 'bold', fontSize: 13}} 
            disabled={!editable} 
            onChange={(e) => setName(e.currentTarget.value)} 
            value={name}
        />
        <Button 
            style={{height: 30, width: '10%'}} 
            onClick={tryEditName} 
            icon={<EditOutlined/>}
        />
    </div>
}

export default QuestName;