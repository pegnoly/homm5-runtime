import { Button, Input, InputRef, Modal, Select, Typography } from "antd";
import { useRef, useState } from "react";
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSpeakersStore } from "../../stores/SpeakersStore";
import { Dialog, useDialogsStore } from "../../stores/DialogsStore";

function DialogCreator() {

    const addDialog = useDialogsStore((state) => state.add_dialog)
    const setCurrentDialogId = useCurrentDialogStore((state) => state.set_current_dialog_id)
    const speakers = useSpeakersStore((state) => state.speakers)

    const [open, setOpen] = useState<boolean>(false)
    const [selectedDirectory, setSelectedDirectory] = useState<string>("");
    const [selectedSpeakers, setSelectedSpeakers] = useState<string[]>([]);

    const nameRef = useRef<InputRef | null>(null)
    const scriptNameRef = useRef<InputRef | null>(null)

    async function close() {
        setOpen(false)
    }

    async function create() {
        setOpen(false)
        await invoke<Dialog>("create_new_dialog", {
            name: nameRef.current?.input?.value,
            scriptName: scriptNameRef.current?.input?.value,
            directory: selectedDirectory,
            speakers: selectedSpeakers
        }).then((dialog) => {
            addDialog(dialog)
            setCurrentDialogId(dialog.id)
        })
    }

    async function pickDirectory() {
        await invoke("pick_dialog_directory")
    }

    listen<string>("dialog_directory_picked", (e) => {
        setSelectedDirectory(e.payload)
    })

    return <>
        <Button
            onClick={() => setOpen(true)}
        >Create new dialog</Button>
        <Modal
            centered={true}
            open={open}
            onCancel={close}
            onClose={close}
            onOk={create}
        >           
            <div style={{width: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 5}}>
                <div style={{width: '70%', display: 'flex', flexDirection: 'row', gap: 5}}>
                    <Typography.Text>Dialog name: </Typography.Text>
                    <Input ref={nameRef}/>
                </div>
                <div style={{width: '70%', display: 'flex', flexDirection: 'row', gap: 5}}>
                    <Typography.Text>Dialog script name: </Typography.Text>
                    <Input ref={scriptNameRef}/>
                </div>
                <Button onClick={pickDirectory}>Pick dialog directory</Button>
                <Typography.Text>{selectedDirectory}</Typography.Text>
                <Typography.Text>Select speakers: </Typography.Text>
                <Select 
                    style={{width: '70%'}}
                    onChange={setSelectedSpeakers}
                    mode="multiple"
                >{speakers.map((s, i) => (
                    <Select.Option key={i} value={s.id}>{s.name}</Select.Option>
                ))}</Select>
            </div>
        </Modal>
    </>
}

export default DialogCreator;