import { useEffect, useState } from "react";
import { BankVariantModel } from "./BankVariants";
import { invoke } from "@tauri-apps/api/core";
import { Select, Typography } from "antd";
import VariantCreaturesSlots from "./BankCreaturesInfo";
import { BankDifficultyType, difficultiesData } from "./BankDifficulties";

function BankVariantFocused(params: {variantId: number | undefined}) {
    
    const [variant, setVariant] = useState<BankVariantModel | null>(null);

    useEffect(() => {
        if (params.variantId != undefined) {
            invoke<BankVariantModel | null>("load_bank_variant", {id: params.variantId})
                .then((data) => setVariant(data));
        }
    }, [params.variantId])

    async function updateVariantChance(newDifficulty: BankDifficultyType) {
        await invoke("update_bank_variant_difficulty", {id: params.variantId, difficulty: newDifficulty})
            .then(() => setVariant({...variant!, difficulty: newDifficulty}));
    }

    return <>{
        variant == null ? 
        null :
        <div>
            <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center', height: '50%', paddingTop: '10%'}}>
                <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 20, color: 'darkorchid'}}>Current variant</Typography.Text>
                <Typography.Text style={{fontFamily: 'cursive', fontSize: 16, fontWeight: 'bold'}}>Variant difficulty</Typography.Text>
                <Select 
                    value={variant.difficulty}
                    onChange={updateVariantChance}
                >{Array.from(difficultiesData.entries()).map((value, index) => (
                    <Select.Option key={index} value={value[0]}>{value[1]}</Select.Option>
                ))}</Select>
                <VariantCreaturesSlots variantId={variant.id}/>
            </div>
        </div>
    }</>
}

export default BankVariantFocused;