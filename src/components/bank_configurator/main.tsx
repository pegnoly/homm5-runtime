import { invoke } from "@tauri-apps/api/core";
import { Button, Carousel, Col, Grid, Input, InputRef, List, Modal, Segmented, Select, Typography } from "antd";
import { useEffect, useRef, useState } from "react";
import { Link, Route, Router, Routes, useParams } from "react-router";

enum BankType {
    Crypt = "BTD_BANK_CRYPT",
    Pyramid = "BTD_BANK_PYRAMID",
    MagiVault = "BTD_BANK_MAGI_VAULT",
    DragonUtopia = "BTD_BANK_DRAGON_UTOPIA",
    ElementalStockpile = "BTD_BANK_ELEMENTAL_STOCKPILE",
    DwarvenTreasure = "BTD_BANK_DWARVEN_TREASURE",
    BloodTemple = "BTD_BANK_BLOOD_TEMPLE",
    TreantThicket = "BTD_BANK_TREANT_THICKET",
    GargoyleStonevault = "BTD_BANK_GARGOYLE_STONEVAULT",
    SunkenTemple = "BTD_BANK_SUNKEN_TEMPLE"
}

type BankModel = {
    id: number,
    type: BankType,
    name: string,
    recharge_timer: number,
    recharge_count: number,
    luck_loss: number,
    morale_loss: number
}

function BanksConfiguratorMain() {
    const [banks, setBanks] = useState<BankModel[]>([]);

    useEffect(() => {
        invoke<BankModel[]>("get_all_banks")
            .then((data) => setBanks(data));
    }, [])

    return <>
        <Routes>
            <Route path="/" element={<BanksList banks={banks}/>}/>
            <Route path="/bank/:id" element={<BankFocused/>}/>
        </Routes>
    </>
}

function BanksList(props: {banks: BankModel[]}) {
    return <>
        <List>{props.banks.map((b, i) => (
            <Link key={i} to={`bank/${b.id}`}>
                <List.Item>{b.name}</List.Item>
            </Link>
        ))}</List>
    </>
}

function BankFocused() {
    const { id } = useParams();

    const [bank, setBank] = useState<BankModel | null>(null);
    const [currentVariant, setCurrentVariant] = useState<number | null>();

    function variantSelected(newVariant: number) {
        setCurrentVariant(newVariant);
    }

    useEffect(() => {
        if (id != undefined) {
            invoke<BankModel>("load_bank", {id: parseInt(id)})
                .then((data) => setBank(data));
        }
    }, [id])

    return <div style={{paddingLeft: '5%'}}>
        <div style={{justifyContent: 'center', display: 'flex'}}>
            <Typography.Text style={{textAlign: 'center', fontFamily: 'cursive', fontWeight: 'bold', fontSize: 20}}>{bank?.name}</Typography.Text>
        </div>
        <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
            <Col span={11}>
                <BankProps bank={bank}/>
                <BankVariants bankId={bank?.id} onVariantSelected={variantSelected}/>
            </Col>
            <Col span={11}>{
                currentVariant != null ? <BankVariantFocused variantId={currentVariant}/> : null
            }</Col>
        </div>
    </div>
}

function BankProps(data: {bank: BankModel | null}) {
    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
            <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Props</Typography.Text>
            <BankProp text="Recharges count" value={data.bank?.recharge_count}/>
            <BankProp text="Recharge timer" value={data.bank?.recharge_timer}/>
            <BankProp text="Luck loss" value={data.bank?.luck_loss}/>
            <BankProp text="Morale loss" value={data.bank?.morale_loss}/>
        </div>
    )
}

function BankProp(data: {text: string, value: any}) {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold'}}>{`${data.text}: `}</Typography.Text>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold', color: 'green'}}>{data.value}</Typography.Text>
    </div>
}

enum BankDifficultyType {
    Easy = "BANK_DIFFICULTY_EASY",
    Medium = "BANK_DIFFICULTY_MEDIUM",
    Hard = "BANK_DIFFICULTY_HARD",
    Critical = "BANK_DIFFICULTY_CRITICAL",
    Boss = "BANK_DIFFICULTY_BOSS"
}

type BankVariantModel = {
    id: number,
    chance: number,
    difficulty: BankDifficultyType
}

const difficultyNames = new Map<BankDifficultyType, string>([
    [BankDifficultyType.Easy, "Easy"],
    [BankDifficultyType.Medium, "Medium"],
    [BankDifficultyType.Hard, "Hard"],
    [BankDifficultyType.Critical, "Critical"],
    [BankDifficultyType.Boss, "Boss"]
]);

function BankVariants(data: {bankId: number | undefined, onVariantSelected: (variant: number) => void}) {

    const [open, setOpen] = useState<boolean>(false);
    const variantChanceRef = useRef<InputRef | null>(null);
    const [diff, setDiff] = useState<BankDifficultyType | undefined>(undefined);
    const [variants, setVariants] = useState<BankVariantModel[]>([]);

    useEffect(() => {
        if (data.bankId != undefined) {
            invoke<BankVariantModel[]>("load_bank_variants", {bankId: data.bankId})
                .then((data) => {
                    console.log("Variants: ", data);
                    setVariants(data);
                });
        }
    }, [data.bankId])

    function close() {
        setOpen(false)
    }

    function updateDiff(newDiff: BankDifficultyType) {
        setDiff(newDiff);
    }
    
    async function createVariant() {
        setOpen(false);
        await invoke<BankVariantModel | null>("create_variant", {bankId: data.bankId, chance: parseInt(variantChanceRef.current?.input?.value!), difficulty: diff})
            .then((data) => {
                console.log("Created: ", data);
                setVariants([...variants, data!])
            });
    }

    async function selectVariant(variant: number) {
        data.onVariantSelected(variant);
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center', paddingTop: '10%'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Variants</Typography.Text>
        <Button onClick={() => setOpen(true)}>Create variant</Button>
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 14}}>Existing</Typography.Text>
        <Segmented 
            vertical 
            options={variants.map((v) => ({value: v.id, label: difficultyNames.get(v.difficulty)}))}
            onChange={selectVariant}
        />
        <Modal
            open={open}
            onCancel={close}
            onClose={close}
            onOk={createVariant}
            centered
        >
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <Typography.Text>Input variant chance</Typography.Text>
                <Input type="number" ref={variantChanceRef}/>
                <Typography.Text>Select variant difficulty</Typography.Text>
                <Select value={diff} onChange={updateDiff}>
                    <Select.Option key={0} value={BankDifficultyType.Easy}>Easy</Select.Option>
                    <Select.Option key={1} value={BankDifficultyType.Medium}>Medium</Select.Option>
                    <Select.Option key={2} value={BankDifficultyType.Hard}>Hard</Select.Option>
                    <Select.Option key={3} value={BankDifficultyType.Critical}>Critical</Select.Option>
                    <Select.Option key={4} value={BankDifficultyType.Boss}>Boss</Select.Option>
                </Select>
            </div>
        </Modal>
    </div>
}

function BankVariantFocused(data: {variantId: number}) {
    
    const [variant, setVariant] = useState<BankVariantModel | null>(null);

    useEffect(() => {
        invoke<BankVariantModel | null>("load_variant", {id: data.variantId})
            .then((data) => setVariant(data));
    }, [data.variantId])

    return <>{
        variant == null ? 
        null :
        <div>
            <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
                <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Current variant</Typography.Text>
                <BankProp text="Chance" value={variant.chance}/>
            </div>
            <VariantCreaturesInfo variantId={variant.id}/>
        </div>
    }</>
}

enum CreatureSlotType {
    Tier = "CREATURE_SLOT_TYPE_TIER",
    Concrete = "CREATURE_SLOT_TYPE_CONCRETE"
}

type CreatureSlotData = {
    id: number,
    base_power: number | null,
    power_grow: number | null,
    creature_tier: number | null,
    creature_town: number | null,
    creature_id: number | null,
    creature_count: number | null
}

function VariantCreaturesInfo(data: {variantId: number}) {
    
    const [slotsIds, setSlotsIds] = useState<number []>([]);
    const [open, setOpen] = useState<boolean>(false);
    const [slotType, setSlotType] = useState<CreatureSlotType | null>(null);

    function close() {
        setOpen(false);
    }

    useEffect(() => {
        invoke<number[]>("load_creature_slots_ids", {variantId: data.variantId})
            .then((data) => {
                setSlotsIds(data);
            })
    }, [data.variantId])

    async function createSlot() {
        setOpen(false);
        invoke<number>("create_creature_slot", {variantId: data.variantId, slotType: slotType})
            .then((data) => {
                setSlotsIds([...slotsIds, data]);
            })
    }

    return <div style={{paddingTop: '10%'}}>
        <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
            <Typography.Text style={{fontFamily: 'cursive', fontStretch: 'expanded'}}>Creatures info</Typography.Text>
            <Button onClick={() => setOpen(true)}>Create new slot</Button>
        </div>
        <Carousel arrows vertical>{slotsIds.map((id, i) => (
            <CreatureSlot key={i} id={id}/>
        ))}</Carousel>
        <Modal
            open={open}
            onClose={close}
            onCancel={close}
            onOk={() => createSlot()}
        >
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <Typography.Text>Type of slot</Typography.Text>
                <Select onChange={setSlotType} value={slotType}>
                    <Select.Option key={0} value={CreatureSlotType.Tier}>Creature tier</Select.Option>
                    <Select.Option key={1} value={CreatureSlotType.Concrete}>Concrete creature</Select.Option>
                </Select>
            </div>
        </Modal>
    </div>
}

function CreatureSlot(data: {id: number}) {

    const [slotData, setSlotData] = useState<CreatureSlotData | null>(null);

    useEffect(() => {
        invoke<CreatureSlotData>("load_creature_slot", {id: data.id})
            .then((data) => {
                console.log("Slot loaded: ", data);
                setSlotData(data);
            });
    }, [data.id]);
    
    return <div style={{display: 'flex', flexDirection: 'column', padding: '15%'}}>
        {slotData?.base_power == null ? null : <Typography.Text>{`Power: ${slotData.base_power}`}</Typography.Text>}
        {slotData?.creature_town == null ? null : <Typography.Text>{`Town: ${slotData.creature_town}`}</Typography.Text>}
        {slotData?.creature_id == null ? null : <Typography.Text>{`ID: ${slotData.creature_id}`}</Typography.Text>}
    </div>
}

export default BanksConfiguratorMain;

