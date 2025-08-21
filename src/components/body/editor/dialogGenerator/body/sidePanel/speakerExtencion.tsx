import { ActionIcon, Group, Select } from "@mantine/core";
import { useState } from "react";
import { useCurrentDialogId, useDialogActions, useDialogSpeakers, useSpeakers } from "../../store";
import { useMutation } from "@tanstack/react-query";
import { DialogGeneratorApi } from "../../api";
import { IconPlus } from "@tabler/icons-react";

function DialogSpeakerExtencion() {
    const speakers = useSpeakers();
    const currentDialogId = useCurrentDialogId();
    const currentDialogSpeakers = useDialogSpeakers();
    const actions = useDialogActions();

    const [selectedSpeaker, setSelectedSpeaker] = useState<number | undefined>(undefined);

    const mutation = useMutation({
        mutationFn: async(data: {id: number, speakerId: number}) => {
            return DialogGeneratorApi.addSpeaker(data.id, data.speakerId);
        },
        onSuccess(_data, variables, _context) {
            setSelectedSpeaker(undefined);
            actions.updateDialogSpeakers([...currentDialogSpeakers!, variables.speakerId])
        },
    })

    return (
        <Group>
            <Select
                radius={0}
                disabled={currentDialogId == undefined}
                label="Select speaker to add"
                placeholder="Speaker name"
                value={selectedSpeaker?.toString()}
                data={speakers?.filter(s => !currentDialogSpeakers?.includes(s.id)).map(s => ({
                    value: s.id.toString(), label: s.name
                }))}
                onChange={(value) => setSelectedSpeaker(parseInt(value!))}
            />
            <ActionIcon 
                radius={0} 
                disabled={selectedSpeaker == undefined} 
                onClick={() => mutation.mutate({id: currentDialogId!, speakerId: selectedSpeaker!})}
            >
                <IconPlus/>
            </ActionIcon>
        </Group>
    )
}

export default DialogSpeakerExtencion;