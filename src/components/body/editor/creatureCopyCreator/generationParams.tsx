import useGameDataStore from "@/stores/GameDataStore";
import { MultiSelect } from "@mantine/core";
import CreatureCopyCreator from "./store";

function CreatureGenerationParams() {
    const abilities = useGameDataStore(state => state.abilities);
    const selectedAbilities = CreatureCopyCreator.useAbilities();
    const actions = CreatureCopyCreator.useActions();

    return (
    <MultiSelect
        label="Add these abilities to generated creatures"
        value={selectedAbilities}
        searchable
        onChange={(value) => actions.updateSelectedAbilities(value)}
        data={abilities.filter(a => a.id != 0).map(a => ({
            label: a.name, value: a.id.toString()
        }))}
    />
    )
}

export default CreatureGenerationParams;