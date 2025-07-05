import useGameDataStore from "@/stores/GameDataStore";
import { Group, MultiSelect, Select, TextInput } from "@mantine/core";
import { CreatableCreatureModel } from "./types";
import CreatureCopyCreator from "./store";

function CreatableCreatureItem({model}: {model: CreatableCreatureModel}) {
    const creatures = useGameDataStore(state => state.creatures);

    const ids = CreatureCopyCreator.useIds();
    const actions = CreatureCopyCreator.useActions();

    if (model.base_creature != undefined && !ids.includes(model.base_creature)) {
        ids.push(model.base_creature);
    }

    return (
    <Group gap="sm" pl="sm" justify="space-around">
        <Select
            size="xs"
            label="Initial creature"
            value={model.base_creature?.toString()}
            searchable
            onChange={(value) => actions.updateModel({...model, base_creature: parseInt(value!)})}
            data={creatures.filter(cr => cr.id > 0 && cr.id < 180).map(cr => ({
                label: cr.name, value: cr.id.toString()
            }))}
        />
        <Select
            size="xs"
            label="Parent creature"
            value={model.parent_creature?.toString()}
            onChange={(value) => actions.updateModel({...model, parent_creature: parseInt(value!)})}
            data={ids.map(id => ({
                value: id.toString(), label: id > 179 ? id.toString() : creatures.find(c => c.id == id)?.name! 
            }))}
        />
        <MultiSelect
            maxValues={2}
            miw={150}
            size="xs"
            label="Upgrades"
            value={model.upgrades.map(u => u.toString())}
            onChange={(values) => actions.updateModel({...model, upgrades: values.map(v => parseInt(v))})}
            data={ids.map(id => ({
                value: id.toString(), label: id.toString()
            }))}
        />
        <TextInput
            size="xs"
            label="Inner name"
            value={model.inner_name}
            onChange={(e) => actions.updateModel({...model, inner_name: e.currentTarget.value})}
        />
    </Group>
    )
}

export default CreatableCreatureItem;