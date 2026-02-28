import { Select } from "@mantine/core";
import { useCurrentDialogVariantSpeaker, useDialogActions, useDialogSpeakers, useSpeakers } from "../../store";
import DialogSpeakerExtension from "./speakerExtension";

function DialogStepSpeakerSelector() {
    const currentSpeaker = useCurrentDialogVariantSpeaker();
    const actions = useDialogActions();
    const availableSpeakers = useSpeakers();
    const dialogSpeakers = useDialogSpeakers();

    async function selectSpeaker(value: number) {
        actions.setCurrentVariantSpeaker(value);
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '1%'}}>
            <Select
                radius={0}
                label="Select speaker"
                placeholder="Speaker name"
                searchable
                value={currentSpeaker == undefined ? null : currentSpeaker.toString()}
                onChange={(value) => selectSpeaker(parseInt(value!))}
                data={availableSpeakers?.filter(s => dialogSpeakers?.includes(s.id)).map(s => ({
                    value: s.id.toString(), label: s.name
                }))}
            />
            <DialogSpeakerExtension/>
        </div>
    )
}

export default DialogStepSpeakerSelector;