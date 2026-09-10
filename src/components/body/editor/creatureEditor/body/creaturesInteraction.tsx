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
        <div style={{width: '60%', display: 'flex', flexDirection: 'column', gap: '1.5%'}}>
            <Text style={{textAlign: 'center', fontSize: 20}}>Связи с другими существами</Text>
            <Select
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Базовое существо"
                value={currentCreature.base_creature == "CREATURE_UNKNOWN" ? null : currentCreature.base_creature}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreatureBaseCreature(value!)}   
            />
            <Select
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Парное существо"
                value={currentCreature.pair_creature == "CREATURE_UNKNOWN" ? null : currentCreature.pair_creature}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreaturePairCreature(value!)}    
            />
            <MultiSelect
                size="xs"
                searchable
                style={{width: '100%'}}
                radius={0}
                label="Улучшения"
                value={currentCreature.upgrades.upgrades}
                data={creatures.map(c => ({label: `${c.inner_name != null && c.inner_name.length != 0 ? c.inner_name : c.name} [${c.id}]`, value: c.game_id}))}
                onChange={(value) => updateCreatureUpgrades(value!)}   
            />
            <div style={{width: '100%'}}>
                <Text style={{textAlign: 'center', fontSize: 20}}>Города</Text>
                <div style={{display: "flex", flexDirection: "row", justifyContent: "center", gap: '10%'}}>
                    <Select
                        searchable
                        radius={0}
                        w={200}
                        size="xs"
                        label="Город с точки зрения игры"
                        value={currentCreature.town}
                        onChange={(value) => updateCreatureTown(value as TownType)}
                        data={[
                            {value: TownType.TownAcademy, label: "Академия волшебства"},
                            {value: TownType.TownDungeon, label: "Лига теней"},
                            {value: TownType.TownHeaven, label: "Орден порядка"},
                            {value: TownType.TownInferno, label: "Инферно"},
                            {value: TownType.TownFortress, label: "Северные кланы"},
                            {value: TownType.TownPreserve, label: "Лесной союз"},
                            {value: TownType.TownNecromancy, label: "Некрополис"},
                            {value: TownType.TownStronghold, label: "Великая орда"},
                            {value: TownType.TownNoType, label: "Нейтралы"},
                            {value: TownType.TownSpecial, label: "Специальный"}
                        ]}
                    />
                    <Select
                        searchable
                        radius={0}
                        w={200}
                        size="xs"
                        label="Город с точки зрения скриптов"
                        value={currentCreature.town_extended}
                        onChange={(value) => updateCreatureTownExtended(value as TownTypeExtended)}
                        data={[
                            {value: TownTypeExtended.TownAcademy, label: "Академия волшебства"},
                            {value: TownTypeExtended.TownDungeon, label: "Лига теней"},
                            {value: TownTypeExtended.TownHeaven, label: "Орден порядка"},
                            {value: TownTypeExtended.TownInferno, label: "Инферно"},
                            {value: TownTypeExtended.TownFortress, label: "Северные кланы"},
                            {value: TownTypeExtended.TownPreserve, label: "Лесной союз"},
                            {value: TownTypeExtended.TownNecromancy, label: "Некрополис"},
                            {value: TownTypeExtended.TownStronghold, label: "Великая орда"},
                            {value: TownTypeExtended.TownBastion, label: "Твердыня"},
                            {value: TownTypeExtended.TownSanctuary, label: "Святилище"},
                            {value: TownTypeExtended.TownRenegades, label: "Ренегаты"},
                            {value: TownTypeExtended.TownNoType, label: "Нейтралы"},
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