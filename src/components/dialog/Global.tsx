import { Button, Input, InputRef, Typography } from "antd";
import DialogFilesGenerator from "./Generate";
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore";
import { useShallow } from "zustand/shallow";
import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

function DialogGeneratorGlobals() {

    return <>
        <CurrentDialogData/>
        <CurrentDialogScriptName/>
        <LabelCreator/>
        <DialogFilesGenerator/>
    </>
}

function CurrentDialogData() {

    return <div style={{display: 'flex', flexDirection: 'column', justifyContent: 'center'}}>
        <CurrentDialogName/>
    </div>
}

function CurrentDialogName() {

    const [currentDialogId, setCurrentDialogName] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_id, state.set_current_dialog_name
    ]))

    useEffect(() => {
        if (currentDialogId != null) {
            loadName()
        }
    }, [currentDialogId])

    const loadName = async () => {
        await invoke<string>("load_dialog_name", {dialogId: currentDialogId}).then((name) => setCurrentDialogName(name))
    }

    return <div style={{display: 'flex', flexDirection: 'row', gap: 3}}>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 15}}
        >Current dialog name: </Typography.Text>
        <Name/>
    </div>
}

function Name() {

    const name = useCurrentDialogStore((state) => state.current_dialog_name)

    return <>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 15, color: !name ? 'red' : 'green'}}
        >{!name ? "-" : name}</Typography.Text>
    </>
}

function CurrentDialogScriptName() {
    
    const [currentDialogId, setCurrentDialogScriptName] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_id, state.set_current_dialog_script_name
    ]))

    useEffect(() => {
        if (currentDialogId != null) {
            loadScriptName()
        }
    }, [currentDialogId])

    const loadScriptName = async () => {
        await invoke<string>("load_dialog_script_name", {dialogId: currentDialogId}).then((name) => setCurrentDialogScriptName(name))
    }

    return <div style={{display: 'flex', flexDirection: 'row', gap: 3}}>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 15}}
        >Current dialog script: </Typography.Text>
        <ScriptName/>
    </div>
}

function ScriptName() {

    const scriptName = useCurrentDialogStore((state) => state.current_dialog_script_name)

    return <>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 15, color: !scriptName ? 'red' : 'green'}}
        >{!scriptName ? "-" : scriptName}</Typography.Text>
    </>
}

function LabelCreator() {

    const addLabel = useCurrentDialogStore((state) => state.add_label)

    const labelInputRef = useRef<InputRef | null>(null)

    return <>
        <Input ref={labelInputRef}/>
        <Button
            onClick={() => addLabel(labelInputRef.current?.input?.value!)}
        >Add new label</Button>
    </>
}

export default DialogGeneratorGlobals;