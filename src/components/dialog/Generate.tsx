import { Button } from "antd";
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore";
import { invoke } from "@tauri-apps/api/core";

function DialogFilesGenerator() {

    const currentDialogId = useCurrentDialogStore((state) => state.current_dialog_id)

    return <>
        <Button 
            disabled={!currentDialogId}
            onClick={() => invoke("generate_dialog", {dialogId: currentDialogId})}
            size="large"
        >Generate Lua code</Button>
    </>
}

export default DialogFilesGenerator;