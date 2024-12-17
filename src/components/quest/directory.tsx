import { useShallow } from "zustand/shallow"
import { useQuestStore } from "../../stores/QuestStore"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { Button, Typography } from "antd"
import { EditOutlined } from "@ant-design/icons"

function QuestDirectory() {
    
    const [id, directory, setDirectory] = useQuestStore(useShallow((state) => [state.id, state.directory, state.set_directory]))

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
            style={{fontFamily: 'fantasy', position: 'relative', bottom: -3}}>Папка: </Typography.Text>
        <Typography.Text style={{maxWidth: '75%', maxHeight: 25, fontFamily: 'cursive', fontWeight: 'bold', fontSize: 13}}>{directory}</Typography.Text>
        <Button 
            style={{height: 30, width: '10%'}} 
            onClick={tryUpdateDirectory} 
            icon={<EditOutlined/>}
        />
    </div>
}

export default QuestDirectory;