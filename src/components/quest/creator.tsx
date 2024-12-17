import { Button, Input, Modal, Typography } from "antd";
import { useState } from "react";
import { useQuestCreationContext } from "../../contexts/questCreation";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { EditOutlined } from "@ant-design/icons";
import { useQuestStore } from "../../stores/QuestStore";
import { useShallow } from "zustand/shallow";
import { Quest } from "../../types";

function QuestCreator() {

    const [open, setOpen] = useState<boolean>(false);

    const [setId, setDirectory, setName, setScriptName, setDesc] = useQuestStore(useShallow((state) => [
        state.set_id,
        state.set_directory,
        state.set_name,
        state.set_script_name,
        state.set_desc
    ]));

    const questCreationContext = useQuestCreationContext();

    async function close() {
        setOpen(false)
    }

    async function create() {
        setOpen(false)
        await invoke("create_quest", {
            directory: questCreationContext?.state.directory,
            scriptName: questCreationContext?.state.script_name,
            name: questCreationContext?.state.name
        })
        .then((q) => {
            const quest = q as Quest
            setId(quest.id)
            setName(quest.name)
            setDesc(quest.desc)
            setDirectory(quest.directory)
            setScriptName(quest.script_name)
        })
    }

    return <div style={{width: '45%'}}>
        <Button
            onClick={() => setOpen(true)} 
            style={{height: 25}}
        >Создать квест</Button>
        <Modal
            centered={true}
            open={open}
            onCancel={close}
            onClose={close}
            onOk={create}
        >
            <div style={{display: 'flex', flexDirection: 'column', gap: 5}}>
                <DirectoryPicker/>
                <ScriptNameCreator/>
                <NameCreator/>
            </div>
        </Modal>
    </div>
}

function DirectoryPicker() {

    const questCreationContext = useQuestCreationContext();

    async function pickDirectory() {
        invoke("pick_quest_directory", {initial: true})
    }

    listen<string>("quest_directory_picked", (event) => {
        questCreationContext?.setState({
            ...questCreationContext.state,
            directory: event.payload
        })
    })

    return <div style={{display: 'flex', flexDirection: 'row', gap: 3}}>
        <Typography.Text style={{fontWeight: 'bold'}}>Папка: </Typography.Text>
        <Typography.Text>{questCreationContext?.state.directory}</Typography.Text>
        <Button 
            style={{height: 30, width: '10%'}} 
            onClick={pickDirectory} 
            icon={<EditOutlined/>}
        />
    </div>

}

function ScriptNameCreator() {
    
    const questCreationContext = useQuestCreationContext();

    return <div style={{display: 'flex', flexDirection: 'row', gap: 3}}>
        <div style={{width: '20%'}}>
            <Typography.Text style={{fontWeight: 'bold'}}>Скрипт. имя:</Typography.Text>
        </div>
        <div style={{width: '70%'}}>
            <Input 
                value={questCreationContext?.state.script_name}
                onChange={(e) => questCreationContext?.setState({...questCreationContext.state, script_name: e.currentTarget.value})}
            />
        </div>
    </div>
}

function NameCreator() {
    
    const questCreationContext = useQuestCreationContext();

    return <div style={{display: 'flex', flexDirection: 'row', gap: 3}}>
        <div style={{width: '20%'}}>
            <Typography.Text style={{fontWeight: 'bold'}}>Имя:</Typography.Text>
        </div>
        <div style={{width: '70%'}}>
            <Input 
                value={questCreationContext?.state.name}
                onChange={(e) => questCreationContext?.setState({...questCreationContext.state, name: e.currentTarget.value})}
            />
        </div>
    </div>
}

export default QuestCreator;