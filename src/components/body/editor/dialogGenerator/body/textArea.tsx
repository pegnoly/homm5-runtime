import { Textarea } from "@mantine/core";
import { useCurrentDialogVariantText, useDialogActions } from "../store";

function DialogGeneratorTextArea() {
    const currentVariantText = useCurrentDialogVariantText();
    const actions = useDialogActions();

    return (
    <Textarea 
        style={{padding: "4%"}}
        radius={0}
        rows={20}
        minRows={20}
        maxRows={25}
        value={currentVariantText}
        onChange={(e) => {
            actions.setCurrentVariantText(e.currentTarget.value);
            actions.setCurrentVariantSaved(false);
        }}
    />
    )
}

export default DialogGeneratorTextArea;