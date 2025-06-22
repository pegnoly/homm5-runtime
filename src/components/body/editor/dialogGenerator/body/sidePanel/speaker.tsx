import { Select } from "@mantine/core";
import { useCurrentDialogVariantSpeaker, useDialogActions, useDialogSpeakers, useSpeakers } from "../../store";

function DialogStepSpeakerSelector() {
    const currentSpeaker = useCurrentDialogVariantSpeaker();
    const actions = useDialogActions();
    const availableSpeakers = useSpeakers();
    const dialogSpeakers = useDialogSpeakers();

    async function selectSpeaker(value: number) {
        actions.setCurrentVariantSpeaker(value);
    }

    return (
    <Select
        label="Select speaker"
        placeholder="Speaker name"
        value={currentSpeaker?.toString()}
        onChange={(value) => selectSpeaker(parseInt(value!))}
        data={availableSpeakers?.filter(s => dialogSpeakers?.includes(s.id)).map(s => ({
            value: s.id.toString(), label: s.name
        }))}
    />
    )
}

export default DialogStepSpeakerSelector;