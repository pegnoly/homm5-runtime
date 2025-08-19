import { Button } from "@mantine/core";
import { useCurrentDialogVariantId, useCurrentDialogVariantSpeaker, useCurrentDialogVariantText, useCurrentVariantSaved, useDialogActions } from "../../store";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../../api";

export type SaveDialogVariantPayload = {
    id: number,
    speaker: number,
    text: string
}

function DialogStepSaver() {
    const variantId = useCurrentDialogVariantId();
    const speaker = useCurrentDialogVariantSpeaker();
    const text = useCurrentDialogVariantText();
    const isSaved = useCurrentVariantSaved();
    const actions = useDialogActions();

    const mutation = useMutation({
        mutationFn: async(payload: SaveDialogVariantPayload) => {
            return DialogGeneratorApi.saveVariant(payload);
        },
        onSuccess(_data, _variables, _context) {
            actions.setCurrentVariantSaved(true);
        },
    })

    return (
    <>
        <Button
            disabled={isSaved}
            onClick={() => mutation.mutate({id: variantId!, speaker: speaker!, text: text!})} 
            radius={0} 
            size="md"
        >Save variant</Button>
    </> 
    )
}

export default DialogStepSaver;