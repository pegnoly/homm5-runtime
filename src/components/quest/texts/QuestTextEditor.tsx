import { invoke } from "@tauri-apps/api/core"
import { Button, Input, InputRef, Modal, Typography } from "antd"
import TextArea, { TextAreaRef } from "antd/es/input/TextArea"
import { useRef, useState } from "react"

function QuestTextEditor() {
    const [name, setName] = useState<string | null>(null);

    const textRef = useRef<TextAreaRef | null>(null)

    async function saveText() {
        console.log("Text: ", textRef.current?.resizableTextArea?.textArea.value)
        await invoke("save_quest_text", {name: name, text: textRef.current?.resizableTextArea?.textArea.value})
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 4}}>
        <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
            <NewTextCreator onTextCreated={(s) => setName(s)}/>
            <Typography.Text 
                style={{fontFamily: 'cursive', fontSize: 19, fontWeight: 'bold', color: !name ? "red" : "green"}}
            >{!name ? " Not created" : name}</Typography.Text>
        </div>
        <TextArea 
            disabled={!name}
            rows={17}
            ref={textRef}
        />
        <Button
            style={{width: '30%'}}
            disabled={!name} 
            onClick={saveText}
        >Save text</Button>
    </div>
}

function NewTextCreator({onTextCreated}: {onTextCreated: (s: string) => void}) {

    const [open, setOpen] = useState<boolean>(false)
    const inputRef = useRef<InputRef | null>(null)

    function close() {
        setOpen(false)
    }

    function create() {
        setOpen(false)
        onTextCreated(inputRef.current?.input?.value!)
    }

    return <>
        <Button onClick={() => setOpen(true)}>Create new text</Button>
        <Modal
            centered={true}
            open={open}
            onClose={close}
            onCancel={close}
            onOk={create}
        >
            <div style={{display: 'flex', 'flexDirection': 'row', gap: 5}}>
                <Typography.Text>Enter name:</Typography.Text>
                <Input ref={inputRef}/>
            </div>
        </Modal>
    </>
}

export default QuestTextEditor;