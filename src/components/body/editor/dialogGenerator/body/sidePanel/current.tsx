import {MultiSelect, Select, Text} from "@mantine/core";
import {
    DialogGeneratorStore
} from "@/components/body/editor/dialogGenerator/store.ts";

function DialogStepCurrent() {
    const currentSpeakers = DialogGeneratorStore.useCurrentVariantSpeakers();
    const actions = DialogGeneratorStore.useActions();
    const availableSpeakers = DialogGeneratorStore.useSpeakers();
    const dialogSpeakers = DialogGeneratorStore.useDialogSpeakers();
    const labels = DialogGeneratorStore.useDialogLabels();
    const currentLabel = DialogGeneratorStore.useCurrentVariantLabel();
    async function selectSpeakers(value: number[]) {
        actions.setCurrentVariantSpeakers(value);
    }

    async function labelSelected(value: string) {
        actions.setCurrentVariantLabel(value);
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '1%'}}>
            <Text>Настройки текущей реплики</Text>
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
            <Select
                radius={0}
                size="xs"
                searchable
                label="Выберите вариант реплики"
                value={currentLabel}
                onChange={(value) => labelSelected(value!)}
                data={labels?.map(label => ({
                    value: label, label: label
                }))}
            />
        </div>
    )
}

export default DialogStepCurrent;