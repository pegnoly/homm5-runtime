import { invoke } from "@tauri-apps/api/core";
import { Button, Input, InputRef, Modal, Select, Typography } from "antd";
import { useEffect, useRef, useState } from "react";
import { BankDifficultyType, difficultiesData } from "./BankDifficulties";
import BankVariantFocused from "./BankVariantFocused";

export type BankVariantModel = {
    id: number,
    label: string
    difficulty: BankDifficultyType
}

function BankVariants(data: {bankId: number | undefined}) {
    
    const [variants, setVariants] = useState<BankVariantModel[]>([]);
    const [selectedVariantId, setSelectedVariantId] = useState<number | undefined>(undefined);


    useEffect(() => {
        if (data.bankId != undefined) {
            invoke<BankVariantModel[]>("load_bank_variants", {bankId: data.bankId})
                .then((data) => {
                    setVariants(data);
                });
        }
    }, [data.bankId])

    async function variantCreated(created: BankVariantModel | null) {
        setVariants([...variants, created!]);
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center', height: '100%'}}>
        <div style={{height: '50%', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 10}}>
            <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 20, color: 'darkorchid'}}>Variants</Typography.Text>
            <BankVariantCreator bankId={data.bankId!} onVariantCreated={variantCreated}/>
            <Typography.Text style={{fontFamily: 'cursive', fontSize: 16, fontWeight: 'bold'}}>Select variant</Typography.Text>
            <BankVariantSelector variants={variants} selected={selectedVariantId} selectCallback={setSelectedVariantId}/>
        </div>
        <BankVariantFocused variantId={selectedVariantId}/>
    </div>
}

function BankVariantSelector(data: {variants: BankVariantModel[], selected: number | undefined, selectCallback: (newSelected: number) => void}) {

    return <div>
        <Select style={{width: 150}} 
            value={data.selected}
            onChange={data.selectCallback}
        >{data.variants.map((v, i) => (
            <Select.Option key={i} value={v.id}>{v.label}</Select.Option>
        ))}</Select>
    </div>
}

function BankVariantCreator(params: {bankId: number, onVariantCreated: (created: BankVariantModel | null) => void}) {

    const [open, setOpen] = useState<boolean>(false);
    const variantLabelRef = useRef<InputRef | null>(null);
    const [diff, setDiff] = useState<BankDifficultyType | undefined>(undefined);

    function close() {
        setOpen(false)
    }

    function updateDiff(newDiff: BankDifficultyType) {
        setDiff(newDiff);
    }
    
    async function createVariant() {
        setOpen(false);
        await invoke<BankVariantModel | null>("create_bank_variant", {bankId: params.bankId, label: variantLabelRef.current?.input?.value!, difficulty: diff})
            .then((data) => {
                console.log("Created: ", data);
                params.onVariantCreated(data);
            });
    }
    
    return <>
        <Button onClick={() => setOpen(true)}>Create variant</Button>
        <Modal
            open={open}
            onCancel={close}
            onClose={close}
            onOk={createVariant}
            centered
        >
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <Typography.Text>Input variant label</Typography.Text>
                <Input type="text" ref={variantLabelRef}/>
                <Typography.Text>Select variant difficulty</Typography.Text>
                <Select value={diff} onChange={updateDiff}>{Array.from(difficultiesData.entries()).map((value, index) => (
                    <Select.Option key={index} value={value[0]}>{value[1]}</Select.Option>
                ))}</Select>
            </div>
        </Modal>
    </>
}

export default BankVariants;