import { Button } from "@mantine/core";
import {
    DialogGeneratorStore
} from "../../store";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../../api";

export type SaveDialogVariantPayload = {
    id: number,
    speakers: number[],
    text: string
}

function DialogStepSaver() {
    const variantId = DialogGeneratorStore.useCurrentVariantId();
    const speakers = DialogGeneratorStore.useCurrentVariantSpeakers();
    const text = DialogGeneratorStore.useCurrentVariantText();
    const isSaved = DialogGeneratorStore.useCurrentVariantSaved();
    const actions = DialogGeneratorStore.useActions();

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
            onClick={() => mutation.mutate({id: variantId!, speakers: speakers.ids, text: text!})}
            radius={0} 
            size="md"
        >Сохранить вариант реплики</Button>
    </> 
    )
}

export default DialogStepSaver;