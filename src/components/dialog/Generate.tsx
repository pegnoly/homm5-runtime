import { Button } from "antd";
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore";

function DialogFilesGenerator() {

    const currentDialogId = useCurrentDialogStore((state) => state.current_dialog_id)

    return <>
        <Button 
            disabled={!currentDialogId}
            size="large"
        >Generate Lua code</Button>
    </>
}

export default DialogFilesGenerator;