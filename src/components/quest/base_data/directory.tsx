import { useShallow } from "zustand/shallow"
import { useCurrentQuestStore } from "../../../stores/QuestStore"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { Button, Typography } from "antd"
import { EditOutlined } from "@ant-design/icons"
import { useEffect } from "react"

function QuestDirectory() {
    
    const [id, directory, setDirectory] = useCurrentQuestStore(useShallow((state) => [state.id, state.directory, state.set_directory]))

    useEffect(() => {
        if (id != null) {
            loadDir()
        }
    }, [id])

    const loadDir = async () => {
        await invoke<string>("load_quest_directory", {questId: id}).then((res) => setDirectory(res))
    }

    async function tryUpdateDirectory() {
        await invoke("pick_quest_directory", {initial: false})
    }

    listen<string>("quest_directory_updated", (event) => {
        setDirectory(event.payload)
        console.log("id: ", id)
        invoke("update_quest_directory", {questId: id, directory: event.payload})
    })

    return <div style={{display: 'flex', paddingTop: '3%', flexDirection: 'row', gap: 3}}>
        <Typography.Text 
            style={{fontFamily: 'fantasy', position: 'relative', bottom: -3.5}}>Directory: </Typography.Text>
        <Typography.Text 
            style={{maxWidth: '75%', maxHeight: 25, fontFamily: 'cursive', fontWeight: 'bold', fontSize: 13, position: 'relative', bottom: -4}}
        >{directory}</Typography.Text>
        <Button 
            style={{height: 30, width: '10%'}} 
            onClick={tryUpdateDirectory} 
            icon={<EditOutlined/>}
        />
    </div>
}

export default QuestDirectory;