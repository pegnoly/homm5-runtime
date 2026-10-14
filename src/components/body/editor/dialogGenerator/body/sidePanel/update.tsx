import {ActionIcon, Group, Select, Text, TextInput} from "@mantine/core";
import {IconPlus} from "@tabler/icons-react";
import {useState} from "react";
import {useMutation} from "@tanstack/react-query";
import {DialogGeneratorApi} from "@/components/body/editor/dialogGenerator/api.ts";
import {DialogGeneratorStore} from "@/components/body/editor/dialogGenerator/store.ts";

export type UpdateLabelsPayload = {
    dialogId: number,
    labels: string []
}

function DialogUpdater() {
    const actions = DialogGeneratorStore.useActions();
    const dialogId = DialogGeneratorStore.useCurrentDialogId();
    const labels = DialogGeneratorStore.useDialogLabels();
    const speakers = DialogGeneratorStore.useSpeakers();
    const currentDialogSpeakers = DialogGeneratorStore.useDialogSpeakers();

    const [labelName, setLabelName] = useState<string | undefined>(undefined);
    const [selectedSpeaker, setSelectedSpeaker] = useState<number | undefined>(undefined);

    const addLabelMutation = useMutation({
        mutationFn: async(payload: UpdateLabelsPayload) => {
            return DialogGeneratorApi.updateLabels(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.updateDialogLabels(variables.labels);
            setLabelName(undefined);
        },
    });

    const addSpeakerMutation = useMutation({
        mutationFn: async(data: {id: number, speakerId: number}) => {
            return DialogGeneratorApi.addSpeaker(data.id, data.speakerId);
        },
        onSuccess(_data, variables, _context) {
            setSelectedSpeaker(undefined);
            actions.updateDialogSpeakers([...currentDialogSpeakers!, variables.speakerId])
        },
    })

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '1%', alignItems: 'center'}}>
            <Text>Изменения диалога</Text>
            <Group justify="stretch" align="center">
                <TextInput
                    w={170}
                    radius={0}
                    size="xs"
                    label="Создать новый вариант"
                    placeholder="Метка варианта"
                    value={labelName}
                    onChange={(e) => setLabelName(e.currentTarget.value)}
                />
                <ActionIcon
                    radius={0}
                    style={{position: 'relative', top: 11}}
                    disabled={labelName == undefined || labelName.length == 0}
                    onClick={() => addLabelMutation.mutate({dialogId: dialogId!, labels: [...labels!, labelName!]})}
                >
                    <IconPlus/>
                </ActionIcon>
            </Group>
            <Group>
                <Select
                    w={170}
                    size="xs"
                    radius={0}
                    disabled={dialogId == undefined}
                    label="Добавить персонажа"
                    searchable
                    placeholder="Имя персонажа"
                    value={selectedSpeaker?.toString()}
                    data={speakers?.filter(s => !currentDialogSpeakers?.includes(s.id)).map(s => ({
                        value: s.id.toString(), label: s.name
                    }))}
                    onChange={(value) => setSelectedSpeaker(parseInt(value!))}
                />
                <ActionIcon
                    style={{position: 'relative', top: 12}}
                    radius={0}
                    disabled={selectedSpeaker == undefined}
                    onClick={() => addSpeakerMutation.mutate({id: dialogId!, speakerId: selectedSpeaker!})}
                >
                    <IconPlus/>
                </ActionIcon>
            </Group>
        </div>
    )
}

export default DialogUpdater;