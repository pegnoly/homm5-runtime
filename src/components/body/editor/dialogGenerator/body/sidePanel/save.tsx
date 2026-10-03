import { Button } from "@mantine/core";
import {
    useCurrentDialogVariantId,
    useCurrentDialogVariantSpeakers, useCurrentDialogVariantText, useCurrentVariantSaved, useDialogActions
} from "../../store";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../../api";

export type SaveDialogVariantPayload = {
    id: number,
    speakers: number[],
    text: string
}

function DialogStepSaver() {
    const variantId = useCurrentDialogVariantId();
    const speakers = useCurrentDialogVariantSpeakers();
    const text = useCurrentDialogVariantText();
    const isSaved = useCurrentVariantSaved();
    const actions = useDialogActions();

    const mutation = useMutation({
        mutationFn: async(payload: SaveDialogVariantPayload) => {
            console.log("Payload: ", payload)
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
            onClick={() => mutation.mutate({id: variantId!, speakers: speakers.ids, text: text!})}
            radius={0} 
            size="md"
        >Save variant</Button>
    </> 
    )
}

export default DialogStepSaver;