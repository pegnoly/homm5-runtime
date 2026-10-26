import { Select } from "@mantine/core";
import { QuestGeneratorStore } from "../store";
import { useMutation } from "@tanstack/react-query";
import { QuestGeneratorApi } from "../api";

function QuestSelector() {
    const quests = QuestGeneratorStore.useQuests();
    const questId = QuestGeneratorStore.useQuestId();
    const actions = QuestGeneratorStore.useActions();

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
        label="Загрузить существующий квест активной карты"
        placeholder="Выбрать"
        value={questId?.toString()}
        onChange={(value) => mutation.mutate(parseInt(value!))}
        data={quests?.map(q => ({
            label: q.name, value: q.id.toString()
        }))}
    />
    )
}

export default QuestSelector;