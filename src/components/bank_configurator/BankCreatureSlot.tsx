import { invoke } from "@tauri-apps/api/core";
import { Select, Typography } from "antd";
import { useEffect, useState } from "react";
import BankStringProperty from "./utils";

export enum CreatureSlotType {
    Tier = "CREATURE_SLOT_TYPE_TIER",
    Concrete = "CREATURE_SLOT_TYPE_CONCRETE"
}

enum CreatureTownType {
    TownNoType = "TOWN_NO_TYPE",
    TownHeaven = "TOWN_HEAVEN",
    TownInferno = "TOWN_INFERNO",
    TownNecromancy = "TOWN_NECROMANCY",
    TownPreserve = "TOWN_PRESERVE",
    TownDungeon = "TOWN_DUNGEON",
    TownAcademy = "TOWN_ACADEMY",
    TownFortress = "TOWN_FORTRESS",
    TownStronghold = "TOWN_STRONGHOLD"
}

const townsNames = new Map<CreatureTownType, string>([
    [CreatureTownType.TownNoType, "Unknown"],
    [CreatureTownType.TownHeaven, "Heaven"],
    [CreatureTownType.TownInferno, "Inferno"],
    [CreatureTownType.TownNecromancy, "Necropolis"],
    [CreatureTownType.TownPreserve, "Sylvan"],
    [CreatureTownType.TownDungeon, "Dungeon"],
    [CreatureTownType.TownAcademy, "Academy"],
    [CreatureTownType.TownFortress, "Fortress"],
    [CreatureTownType.TownStronghold, "Stronghold"]
]);

type CreatureSlotData = {
    base_power: number | null,
    power_grow: number | null,
    creature_tier: number | null,
    creature_town: CreatureTownType | null,
    creature_id: number | null,
    creature_count: number | null
}

function CreatureSlot(params: {id: number}) {

    const [slotData, setSlotData] = useState<CreatureSlotData | null>(null);

    useEffect(() => {
        loadSlotData();
    }, [params.id]);

    const loadSlotData = async () => {
        await invoke<CreatureSlotData>("load_creature_slot", {id: params.id})
            .then((data) => {
                console.log("Slot loaded: ", data);
                setSlotData(data);
            });
    }
    
    return <>{
        slotData == null ? <h1>NO DATA AVAILABLE</h1> :
        <div style={{display: 'flex', flexDirection: 'column', gap: 10, paddingLeft: '20%', paddingTop: '10%', paddingBottom: '10%'}}>
            {slotData?.base_power == null ? null : <CreatureBasePower slotId={params.id} initial={slotData.base_power}/>}
            {slotData?.power_grow == null ? null : <CreaturePowerGrow slotId={params.id} initial={slotData.power_grow}/>}
            {slotData?.creature_town == null ? null : <CreatureTown slotId={params.id} initial={slotData.creature_town}/>}
            {slotData?.creature_tier == null ? null : <CreatureTier slotId={params.id} initial={slotData.creature_tier}/>}
        </div>
    }</>
}

function CreatureBasePower(params: {slotId: number, initial: number}) {
    const [power, setPower] = useState<number>(params.initial);

    async function updatePower(newPower: string) {
        await invoke<number>("update_creature_slot_base_power", {slotId: params.slotId, power: newPower})
            .then((data) => setPower(data))
            .catch((error) => console.log("Error updating power: ", error));
    }

    return <BankStringProperty initialValue={power} text="Creature base power" updateCallback={updatePower}/>
}

function CreaturePowerGrow(params: {slotId: number, initial: number}) {
    const [grow, setGrow] = useState<number>(params.initial)

    async function updateGrow(newGrow: string) {
        await invoke<number>("update_creature_slot_power_grow", {slotId: params.slotId, grow: newGrow})
            .then((data) => setGrow(data))
            .catch((error) => console.log("Error updating power: ", error));
    }

    return <BankStringProperty initialValue={grow} text="Creature power grow" updateCallback={updateGrow}/>
}

function CreatureTown(params: {slotId: number, initial: CreatureTownType}) {
    const [town, setTown] = useState<CreatureTownType>(params.initial);

    async function updateTown(newTown: CreatureTownType) {
        setTown(newTown);
        await invoke("update_creature_slot_town", {slotId: params.slotId, town: newTown});
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 2}}>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold'}}>Creature town:</Typography.Text>
        <Select size="small" style={{width: '75%'}} value={town} onChange={updateTown}>{Object.keys(CreatureTownType).map((town, index) => (
            <Select.Option key={index} value={town as CreatureTownType}>{townsNames.get(town as CreatureTownType)}</Select.Option>
        ))}</Select>
    </div>
}

function CreatureTier(params: {slotId: number, initial: number}) {
    const [tier, setTier] = useState<number>(params.initial);

    async function updateTier(newTier: number) {
        setTier(newTier);
        await invoke("update_creature_slot_tier", {slotId: params.slotId, tier: newTier});
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 2}}> 
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold'}}>Creature tier:</Typography.Text>
        <Select size="small" style={{width: '75%'}} value={tier} onChange={updateTier}>{[...Array(7).keys()].map((value, index) => (
            <Select.Option key={index} value={value + 1}>{`Tier ${value + 1}`}</Select.Option>
        ))}</Select>
    </div>
}

export default CreatureSlot;