import { useRef, useState } from "react";
import { Speaker, SpeakerType, useSpeakersStore } from "../../../stores/SpeakersStore";
import { Button, Input, InputRef, Modal, Select, Typography } from "antd";
import { invoke } from "@tauri-apps/api/core";
import { HexColorPicker } from "react-colorful";

function SpeakerCreator() {

    const addSpeaker = useSpeakersStore((state) => state.add_speaker)

    const [open, setOpen] = useState<boolean>(false)
    const [selectedType, setSelectedType] = useState<SpeakerType>(SpeakerType.Hero)
    const [color, setColor] = useState<string>("")
    
    const nameRef = useRef<InputRef | null>(null)
    const scriptNameRef = useRef<InputRef | null>(null)

    async function close() {
        setOpen(false)
    }

    async function create() {
        setOpen(false)
        await invoke<Speaker>("create_speaker", {
            name: nameRef.current?.input?.value, 
            scriptName: scriptNameRef.current?.input?.value,
            color: color,
            speakerType: selectedType.toString()
        }).then((sp) => addSpeaker(sp))
    }

    return <>
        <Button 
            onClick={() => setOpen(true)}
        >Create speaker</Button>
        <Modal
            open={open}
            centered={true}
            onCancel={close}
            onClose={close}
            onOk={create}
        >
            <div>
                <div>
                    <Typography.Text>Speaker name</Typography.Text>
                    <Input ref={nameRef}/>
                </div>
                <div>
                    <Typography.Text>Speaker script name</Typography.Text>
                    <Input ref={scriptNameRef}/>
                </div>
                <Select 
                    style={{width: '30%'}}
                    value={selectedType}
                    onChange={setSelectedType}
                >
                    <Select.Option key={SpeakerType.Hero} value={SpeakerType.Hero}>Hero</Select.Option>
                    <Select.Option key={SpeakerType.Creature} value={SpeakerType.Creature}>Creature</Select.Option>
                </Select>
                <HexColorPicker onChange={setColor}/>
            </div>
        </Modal>
    </>

}

export default SpeakerCreator;