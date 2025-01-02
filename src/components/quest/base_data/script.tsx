import { useShallow } from "zustand/shallow";
import { useCurrentQuestStore } from "../../../stores/QuestStore";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input, Typography } from "antd";
import { EditOutlined } from "@ant-design/icons";

function QuestScriptName() {

    const [id, scriptName, setScriptName] = useCurrentQuestStore(useShallow((state) => [state.id, state.script_name, state.set_script_name]))
    const [editable, setEditable] = useState<boolean>(false);

    useEffect(() => {
        if (id != null) {
            loadScriptName()
        }
    }, [id])

    const loadScriptName = async () => {
        await invoke<string>("load_quest_script_name", {questId: id}).then((res) => setScriptName(res))
    }

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
            style={{fontFamily: 'fantasy', position: 'relative', bottom: -3}}>Script name: </Typography.Text>
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