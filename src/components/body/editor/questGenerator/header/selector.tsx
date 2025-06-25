import { Select } from "@mantine/core";
import { useCurrentQuestId, useQuests, useQuestsActions } from "../store";
import { useMutation } from "@tanstack/react-query";
import { QuestGeneratorApi } from "../api";

function QuestSelector() {
    const quests = useQuests();
    const questId = useCurrentQuestId();
    const actions = useQuestsActions();

    const mutation = useMutation({
        mutationFn: async(id: number) => {
            return QuestGeneratorApi.loadQuest(id);
        },
        onSuccess(data, _variables, _context) {
            if (data) {
                actions.loadCurrentQuest(data);
                actions.setCurrentProgressNumber(0);
            }
        },
    })

    return (
    <Select
        label="Load existing quest for current map"
        placeholder="Select quest"
        value={questId?.toString()}
        onChange={(value) => mutation.mutate(parseInt(value!))}
        data={quests?.map(q => ({
            label: q.name, value: q.id.toString()
        }))}
    />
    )
}

export default QuestSelector;