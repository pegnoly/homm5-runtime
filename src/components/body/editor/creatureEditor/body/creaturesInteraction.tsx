import useGameDataStore from "@/stores/GameDataStore";
import { CreatureEditorStore } from "../store";
import { MultiSelect, Select, Text } from "@mantine/core";
import MagicElementUpdater from "./magicElement";
import { invoke } from "@tauri-apps/api/core";
import { ObjectUtils } from "@/lib/utils";

function CreaturesInteractionEditor() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = CreatureEditorStore.useActions();

    const creatures = useGameDataStore(state => state.creatures);
    const abilities = useGameDataStore(state => state.abilities);

    async function updateCreatureBaseCreature(value: string) {
        await invoke(`update_creature_base_creature`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "base_creature", value)
                actions.updateCreature(newModel);
            })
    }

    async function updateCreaturePairCreature(value: string) {
        await invoke(`update_creature_pair_creature`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "pair_creature", value)
                actions.updateCreature(newModel);
            })
    }
    
    async function updateCreatureUpgrades(value: string[]) {
        await invoke(`update_creature_upgrades`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "upgrades", { upgrades: value })
                actions.updateCreature(newModel);
            })
    }

    async function updateCreatureAbilities(value: string[]) {
        await invoke(`update_creature_abilities`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "abilities", { abilities: value })
                actions.updateCreature(newModel);
            })
    }

    return (
    <>
    {
        currentCreature == undefined ? null :
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
            <div style={{width: '100%'}}>
                <MagicElementUpdater/>
            </div>
        </div>
    }
    </>
    )
}

export default CreaturesInteractionEditor;