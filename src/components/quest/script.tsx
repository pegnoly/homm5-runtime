import { useShallow } from "zustand/shallow";
import { useQuestStore } from "../../stores/QuestStore";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input, Typography } from "antd";
import { EditOutlined } from "@ant-design/icons";

function QuestScriptName() {

    const [id, scriptName, setScriptName] = useQuestStore(useShallow((state) => [state.id, state.script_name, state.set_script_name]))
    const [editable, setEditable] = useState<boolean>(false);

    async function tryEditScriptName() {
        if (editable == true) {
            setEditable(false)
            await invoke("update_quest_script_name", {questId: id, scriptName: scriptName})
        }
        else {
            setEditable(true)
        }
    }

    return <div style={{display: 'flex', paddingTop: '3%', flexDirection: 'row', gap: 3}}>
        <Typography.Text 
            style={{fontFamily: 'fantasy', position: 'relative', bottom: -3}}>Скрипт. имя: </Typography.Text>
        <Input 
            style={{height: 30, width: '60%', fontFamily: 'cursive', fontWeight: 'bold', fontSize: 13}} 
            disabled={!editable} 
            onChange={(e) => setScriptName(e.currentTarget.value)} 
            value={scriptName}
        />
        <Button 
            style={{height: 30, width: '10%'}} 
            onClick={tryEditScriptName} 
            icon={<EditOutlined/>}
        />
    </div>
}

export default QuestScriptName;