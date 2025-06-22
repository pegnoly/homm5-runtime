import { ActionIcon, Group, Select, Stack, TextInput } from "@mantine/core";
import { useCurrentDialogId, useCurrentDialogVariantLabel, useDialogActions, useDialogLabels } from "../../store";
import { useState } from "react";
import { IconPlus } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../../api";

export type UpdateLabelsPayload = {
    dialogId: number,
    labels: string []
}

function CurrentDialogLabels() {
    const labels = useDialogLabels();
    const currentLabel = useCurrentDialogVariantLabel();
    const dialogId = useCurrentDialogId();
    const actions = useDialogActions();

    const [labelName, setLabelName] = useState<string | undefined>(undefined);

    const mutation = useMutation({
        mutationFn: async(payload: UpdateLabelsPayload) => {
            return DialogGeneratorApi.updateLabels(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.updateDialogLabels(variables.labels);
            setLabelName(undefined);
        },
    });

    async function labelSelected(value: string) {
        actions.setCurrentVariantLabel(value);
    }

    return (
    <Stack justify="stretch" align="center">
        <Group justify="stretch" align="center">
            <TextInput
                radius={0}
                label="Create new label"
                placeholder="Label name"
                value={labelName}
                onChange={(e) => setLabelName(e.currentTarget.value)}
            />
            <ActionIcon 
                disabled={labelName == undefined || labelName.length == 0}
                onClick={() => mutation.mutate({dialogId: dialogId!, labels: [...labels!, labelName!]})}
            >
                <IconPlus/>
            </ActionIcon>
        </Group>
        <Select
            radius={0}
            label="Select current step label"
            value={currentLabel}
            onChange={(value) => labelSelected(value!)}
            data={labels?.map(label => ({
                value: label, label: label
            }))}
        />
    </Stack>
    )
}

export default CurrentDialogLabels;