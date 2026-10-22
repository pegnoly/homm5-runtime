import { Button, Stack } from "@mantine/core";
import { DialogGeneratorStore } from "./store"
import { invoke } from "@tauri-apps/api/core";

function DialogGeneratorGlobals() {
    const currentDialogId = DialogGeneratorStore.useCurrentDialogId();
    
    return (
        <Stack>
            <Button
                size="xs"
                onClick={() => invoke("generate_dialog", {dialogId: currentDialogId})}
                disabled={currentDialogId == undefined}
                radius={0}
            >
                Сгенерировать текущий диалог
            </Button>
        </Stack>
    )
}

export default DialogGeneratorGlobals;