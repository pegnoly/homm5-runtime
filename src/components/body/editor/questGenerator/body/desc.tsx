import { useState } from "react";
import { useCurrentQuestId, useQuestsActions } from "../store";
import { Button, Group, Textarea } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { UpdateQuestPayloadBase } from "./data";
import { QuestGeneratorApi } from "../api";

function QuestDescription({initial}: {initial: string | undefined}) {
    const currentId = useCurrentQuestId();
    const actions = useQuestsActions();

    const [desc, setDesc] = useState<string | undefined>(initial);
    const [editable, setEditable] = useState<boolean>(false);

    const mutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {desc: string}) => {
            return QuestGeneratorApi.updateQuestDesc(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestDesc(variables.desc)
        },
    });

    return (
    <>
    {
        desc == undefined ? null :
        <>
            <Textarea
                rows={10}
                disabled={!editable}
                label="Quest description"
                value={desc}
                onChange={(e) => setDesc(e.currentTarget.value)}
            />
            <Group justify="end">
                <div hidden={editable}>
                    <Button onClick={() => setEditable(true)} radius={0} size="xs">Edit desc</Button>
                </div>
                <div hidden={!editable}>
                    <Button 
                        radius={0} 
                        size="xs"
                        onClick={() => {
                            setEditable(false);
                            mutation.mutate({desc: desc!, questId: currentId!});
                        }}
                    >Save desc</Button>
                </div>
            </Group>
        </>
    }
    </>
    )
}

export default QuestDescription;