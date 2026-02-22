import { Button, Stack } from "@mantine/core";
import { useCurrentDialogId } from "./store"
import { invoke } from "@tauri-apps/api/core";

function DialogGeneratorGlobals() {
    const currentDialogId = useCurrentDialogId();
    
    return (
        <Stack>
            <Button size="xs" onClick={() => invoke("generate_dialog", {dialogId: currentDialogId})} disabled={currentDialogId == undefined} radius={0}>
                Generate current dialog
            </Button>
        </Stack>
    )
}

export default DialogGeneratorGlobals;