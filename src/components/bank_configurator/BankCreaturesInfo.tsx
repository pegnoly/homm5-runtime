import { useEffect, useState } from "react";
import CreatureSlot, { CreatureSlotType } from "./BankCreatureSlot";
import { Button, Carousel, Modal, Select, Typography } from "antd";
import { invoke } from "@tauri-apps/api/core";

function VariantCreaturesSlots(params: {variantId: number}) {
    
    const [slotsIds, setSlotsIds] = useState<number []>([]);

    useEffect(() => {
        invoke<number[]>("load_creature_slots_ids", {variantId: params.variantId})
            .then((data) => {
                setSlotsIds(data);
            })
    }, [params.variantId]);

    async function slotCreated(newSlotId: number) {
        setSlotsIds([...slotsIds, newSlotId])
    }

    return <div style={{paddingTop: '10%'}}>
        <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
            <Typography.Text style={{fontFamily: 'cursive', fontStretch: 'expanded'}}>Creatures info</Typography.Text>
            <CreatureSlotCreator variantId={params.variantId} onSlotCreated={slotCreated}/>
        </div>
        <Carousel adaptiveHeight arrows vertical>{slotsIds.map((id, i) => (
            <CreatureSlot key={i} id={id}/>
        ))}</Carousel>
    </div>
}

function CreatureSlotCreator(params: {variantId: number, onSlotCreated: (created: number) => void}) {

    const [open, setOpen] = useState<boolean>(false);
    const [slotType, setSlotType] = useState<CreatureSlotType | null>(null);

    function close() {
        setOpen(false);
    }

    async function createSlot() {
        setOpen(false);
        invoke<number>("create_creature_slot", {variantId: params.variantId, slotType: slotType})
            .then((data) => {
                params.onSlotCreated(data);
            });
    }

    return <>
        <Button onClick={() => setOpen(true)}>Create new slot</Button>
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
    </>
}

export default VariantCreaturesSlots;