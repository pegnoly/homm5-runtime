import { MultiSelect, Select, Text } from "@mantine/core";
import { CreatureEditorStore } from "../store";
import EditableProperty from "@/components/common/editableProperty";
import { CreatureEditableModel } from "../types";
import { invoke } from "@tauri-apps/api/core";
import useGameDataStore from "@/stores/GameDataStore";

const creatureNumberProperties = [
    'attack', 
    'defence', 
    'min_damage', 
    'max_damage', 
    'speed', 
    'inititative',
    'health',
    'spell_points',
    'exp',
    'power',
    'tier',
    'grow',
    'size',
    'range',
    'shots'
]

function updateObjectDynamically<T extends object>(
    obj: T,
    path: string,
    value: any
): T {
    const keys = path.split('.');
    const result = { ...obj };
    
    let current: any = result;
    for (let i = 0; i < keys.length - 1; i++) {
        const key = keys[i];
        current[key] = { ...current[key] };
        current = current[key];
    }
    
    current[keys[keys.length - 1]] = value;
    return result;
}

function getFieldValue<T, K extends keyof T>(obj: T, key: K): T[K] {
    return obj[key];
}


function CreatureEditorBody() {
    const creatures = useGameDataStore(state => state.creatures);
    const abilities = useGameDataStore(state => state.abilities);
    const currentCreature = CreatureEditorStore.useCurrent(); 
    const actions = CreatureEditorStore.useActions();

    console.log("Current: ", currentCreature);

    async function updateCreatureNumberParam(paramName: string, value: number) {
        await invoke(`update_creature_${paramName}`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, paramName, value)
                actions.updateCreature(newModel);
            })
    }

    async function updateCreatureBaseCreature(value: string) {
        await invoke(`update_creature_base_creature`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, "base_creature", value)
                actions.updateCreature(newModel);
            })
    }

    async function updateCreaturePairCreature(value: string) {
        await invoke(`update_creature_pair_creature`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, "pair_creature", value)
                actions.updateCreature(newModel);
            })
    }
    
    async function updateCreatureUpgrades(value: string[]) {
        await invoke(`update_creature_upgrades`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, "upgrades", { upgrades: value })
                actions.updateCreature(newModel);
            })
    }

    async function updateCreatureAbilities(value: string[]) {
        await invoke(`update_creature_abilities`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = updateObjectDynamically(currentCreature!, "abilities", { abilities: value })
                actions.updateCreature(newModel);
            })
    }

    return (
    <>
        {
            currentCreature == undefined ? null :
            <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div style={{width: '25%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1.5%', paddingTop: '2%'}}>
                <Text style={{textAlign: 'center', fontSize: 20}}>Numeric params</Text>
                {
                    Object.entries(currentCreature).filter(v => creatureNumberProperties.includes(v[0])).map(v => (
                        <EditableProperty
                            key={v[0]}
                            initialValue={getFieldValue(currentCreature, v[0] as keyof CreatureEditableModel) as number}
                            onSave={(value) => updateCreatureNumberParam(v[0], value as number)}
                            label={v[0]}
                        />
                    ))
                }</div>
                <div style={{width: '30%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1.5%', paddingTop: '2%'}}>
                    <Text style={{textAlign: 'center', fontSize: 20}}>Other creatures interactions</Text>
                    <Select
                        searchable
                        style={{width: '100%'}}
                        radius={0}
                        label="Base creature"
                        value={currentCreature.base_creature == "CREATURE_UNKNOWN" ? null : currentCreature.base_creature}
                        data={creatures.map(c => ({label: `${c.inner_name != null ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                        onChange={(value) => updateCreatureBaseCreature(value!)}   
                    />
                    <Select
                        searchable
                        style={{width: '100%'}}
                        radius={0}
                        label="Pair creature"
                        value={currentCreature.pair_creature == "CREATURE_UNKNOWN" ? null : currentCreature.pair_creature}
                        data={creatures.map(c => ({label: `${c.inner_name != null ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}   
                        onChange={(value) => updateCreaturePairCreature(value!)}    
                    />
                    <MultiSelect
                        searchable
                        style={{width: '100%'}}
                        radius={0}
                        label="Upgrades"
                        value={currentCreature.upgrades.upgrades}
                        data={creatures.map(c => ({label: `${c.inner_name != null ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                        onChange={(value) => updateCreatureUpgrades(value!)}   
                    />
                    <div style={{width: '100%'}}>
                        <Text style={{textAlign: 'center', fontSize: 20}}>Abilities</Text>
                        <MultiSelect
                            searchable
                            style={{width: '100%'}}
                            radius={0}
                            label="Abilities"
                            value={currentCreature.abilities.abilities}
                            data={abilities.map(a => ({label: a.name, value: a.game_id}))}   
                            onChange={(value) => updateCreatureAbilities(value!)}    
                        />
                    </div>
                </div>
            </div>
        }
    </>
    )
}

export default CreatureEditorBody;