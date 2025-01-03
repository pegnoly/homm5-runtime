import { useShallow } from "zustand/shallow";
import { Dialog, useDialogsStore } from "../../stores/DialogsStore";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button, Select } from "antd";
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore";

function DialogLoader() {

    const [dialogs, setDialogs] = useDialogsStore(useShallow((state) => [state.dialogs, state.set_dialogs]))
    const setCurrentDialogId = useCurrentDialogStore((state) => state.set_current_dialog_id)

    const [selectedId, setSelectedId] = useState<string | null>(null)

    useEffect(() => {
        if (dialogs.length == 0) {
            loadDialogs()
        }
    }, [dialogs])

    const loadDialogs = async () => {
        await invoke<Dialog[]>("load_dialogs").then((ds) => setDialogs(ds))
    } 

    return <div style={{display: 'flex', flexDirection: 'column', gap: 5, width: '30%'}}>
        <Select 
            style={{height: '30%'}}
            onChange={setSelectedId}
        >{dialogs.map((d, i) => (
            <Select.Option key={i} value={d.id}>{d.name}</Select.Option>
        ))}</Select>
        <Button
            style={{height: '30%'}}
            onClick={() => setCurrentDialogId(selectedId!)}
        >Load dialog</Button>
    </div>
}

export default DialogLoader;