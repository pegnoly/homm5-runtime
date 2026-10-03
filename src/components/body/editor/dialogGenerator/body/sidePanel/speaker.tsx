import {MultiSelect} from "@mantine/core";
import {
    useCurrentDialogVariantSpeakers, useDialogActions, useDialogSpeakers, useSpeakers
} from "../../store";
import DialogSpeakerExtension from "./speakerExtension";

function DialogStepSpeakerSelector() {
    const currentSpeakers = useCurrentDialogVariantSpeakers();
    const actions = useDialogActions();
    const availableSpeakers = useSpeakers();
    const dialogSpeakers = useDialogSpeakers();

    async function selectSpeakers(value: number[]) {
        actions.setCurrentVariantSpeakers(value);
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '1%'}}>
            <MultiSelect
                size="xs"
                radius={0}
                label="Выберите персонажей для реплики"
                placeholder="Имена персонажей"
                searchable
                value={currentSpeakers.ids.map(s => s.toString())}
                onChange={(value) => selectSpeakers(value.map(s => parseInt(s)))}
                data={availableSpeakers?.filter(s => dialogSpeakers?.includes(s.id)).map(s => ({
                    value: s.id.toString(), label: s.name
                }))}
            />
            <DialogSpeakerExtension/>
        </div>
    )
}

export default DialogStepSpeakerSelector;