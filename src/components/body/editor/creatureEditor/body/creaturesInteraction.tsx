import useGameDataStore from "@/stores/GameDataStore";
import {CreatureEditorStore} from "../store";
import {MultiSelect, Select, Text} from "@mantine/core";
import MagicElementUpdater from "./magicElement";
import {invoke} from "@tauri-apps/api/core";
import {ObjectUtils} from "@/lib/utils";
import {TownType, TownTypeExtended} from "../../fightGenerator/types";

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

    async function updateCreatureTown(value: TownType) {
        await invoke(`update_creature_town`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "town", value)
                actions.updateCreature(newModel);
            })
    }

    async function updateCreatureTownExtended(value: TownTypeExtended) {
        await invoke(`update_creature_town_extended`, {id: currentCreature?.id, value: value})
            .then(() => {
                const newModel = ObjectUtils.updateObjectDynamically(currentCreature!, "town_extended", value)
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
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Base creature"
                value={currentCreature.base_creature == "CREATURE_UNKNOWN" ? null : currentCreature.base_creature}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreatureBaseCreature(value!)}   
            />
            <Select
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Pair creature"
                value={currentCreature.pair_creature == "CREATURE_UNKNOWN" ? null : currentCreature.pair_creature}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreaturePairCreature(value!)}    
            />
            <MultiSelect
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Upgrades"
                value={currentCreature.upgrades.upgrades}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreatureUpgrades(value!)}   
            />
            <div style={{width: '100%'}}>
                <Text style={{textAlign: 'center', fontSize: 20}}>Abilities</Text>
                <MultiSelect
                    size="xs"
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
                <Text style={{textAlign: 'center', fontSize: 20}}>Towns info</Text>
                <div style={{display: "flex", flexDirection: "row", justifyContent: "space-between"}}>
                    <Select
                        searchable
                        radius={0}
                        w={140}
                        size="xs"
                        label="Actual"
                        value={currentCreature.town}
                        onChange={(value) => updateCreatureTown(value as TownType)}
                        data={[
                            {value: TownType.TownAcademy, label: "Academy"},
                            {value: TownType.TownDungeon, label: "Dungeon"},
                            {value: TownType.TownHeaven, label: "Heaven"},
                            {value: TownType.TownInferno, label: "Inferno"},
                            {value: TownType.TownFortress, label: "Fortress"},
                            {value: TownType.TownPreserve, label: "Preserve"},
                            {value: TownType.TownNecromancy, label: "Necromancy"},
                            {value: TownType.TownStronghold, label: "Stronghold"},
                            {value: TownType.TownNoType, label: "Neutral"},
                            {value: TownType.TownSpecial, label: "Special"}
                        ]}
                    />
                    <Select
                        searchable
                        radius={0}
                        w={140}
                        size="xs"
                        label="Extended"
                        value={currentCreature.town_extended}
                        onChange={(value) => updateCreatureTownExtended(value as TownTypeExtended)}
                        data={[
                            {value: TownTypeExtended.TownAcademy, label: "Academy"},
                            {value: TownTypeExtended.TownDungeon, label: "Dungeon"},
                            {value: TownTypeExtended.TownHeaven, label: "Heaven"},
                            {value: TownTypeExtended.TownInferno, label: "Inferno"},
                            {value: TownTypeExtended.TownFortress, label: "Fortress"},
                            {value: TownTypeExtended.TownPreserve, label: "Preserve"},
                            {value: TownTypeExtended.TownNecromancy, label: "Necromancy"},
                            {value: TownTypeExtended.TownStronghold, label: "Stronghold"},
                            {value: TownTypeExtended.TownBastion, label: "Bastion"},
                            {value: TownTypeExtended.TownSanctuary, label: "Sanctuary"},
                            {value: TownTypeExtended.TownRenegades, label: "Renegades"},
                            {value: TownTypeExtended.TownNoType, label: "Neutral"},
                        ]}
                    />
                </div>
                <MagicElementUpdater/>
            </div>
        </div>
    }
    </>
    )
}

export default CreaturesInteractionEditor;