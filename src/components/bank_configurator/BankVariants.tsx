import { invoke } from "@tauri-apps/api/core";
import { Button, Input, InputRef, Modal, Segmented, Select, Typography } from "antd";
import { useEffect, useRef, useState } from "react";

export enum BankDifficultyType {
    Easy = "BANK_DIFFICULTY_EASY",
    Medium = "BANK_DIFFICULTY_MEDIUM",
    Hard = "BANK_DIFFICULTY_HARD",
    Critical = "BANK_DIFFICULTY_CRITICAL",
    Boss = "BANK_DIFFICULTY_BOSS"
}

export type BankVariantModel = {
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

    async function selectVariant(variant: number) {
        data.onVariantSelected(variant);
    }

    async function variantCreated(created: BankVariantModel | null) {
        setVariants([...variants, created!]);
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center', paddingTop: '10%'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Variants</Typography.Text>
        <BankVariantCreator bankId={data.bankId!} onVariantCreated={variantCreated}/>
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 14}}>Existing</Typography.Text>
        <Segmented 
            vertical 
            options={variants.map((v) => ({value: v.id, label: difficultyNames.get(v.difficulty)}))}
            onChange={selectVariant}
        />
    </div>
}

function BankVariantCreator(params: {bankId: number, onVariantCreated: (created: BankVariantModel | null) => void}) {

    const [open, setOpen] = useState<boolean>(false);
    const variantChanceRef = useRef<InputRef | null>(null);
    const [diff, setDiff] = useState<BankDifficultyType | undefined>(undefined);

    function close() {
        setOpen(false)
    }

    function updateDiff(newDiff: BankDifficultyType) {
        setDiff(newDiff);
    }
    
    async function createVariant() {
        setOpen(false);
        await invoke<BankVariantModel | null>("create_bank_variant", {bankId: params.bankId, chance: parseInt(variantChanceRef.current?.input?.value!), difficulty: diff})
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
                <Typography.Text>Input variant chance</Typography.Text>
                <Input type="number" ref={variantChanceRef}/>
                <Typography.Text>Select variant difficulty</Typography.Text>
                <Select value={diff} onChange={updateDiff}>{Object.keys(BankDifficultyType).map((value, index) => (
                    <Select.Option key={index} value={value as BankDifficultyType}>{difficultyNames.get(value as BankDifficultyType)}</Select.Option>
                ))}</Select>
            </div>
        </Modal>
    </>
}

export default BankVariants;