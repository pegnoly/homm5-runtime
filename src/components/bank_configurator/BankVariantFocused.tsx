import { useEffect, useState } from "react";
import { BankVariantModel } from "./BankVariants";
import { invoke } from "@tauri-apps/api/core";
import { Typography } from "antd";
import BankStringProperty from "./utils";
import VariantCreaturesSlots from "./BankCreaturesInfo";

function BankVariantFocused(params: {variantId: number}) {
    
    const [variant, setVariant] = useState<BankVariantModel | null>(null);

    useEffect(() => {
        invoke<BankVariantModel | null>("load_bank_variant", {id: params.variantId})
            .then((data) => setVariant(data));
    }, [params.variantId])

    async function updateVariantChance(newChance: string) {
        await invoke<number>("update_bank_variant_chance", {variantId: params.variantId, chance: newChance})
            .then((data) => setVariant({...variant!, chance: data}));
    }

    return <>{
        variant == null ? 
        null :
        <div>
            <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
                <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Current variant</Typography.Text>
                <BankStringProperty initialValue={variant.chance} text="Variant prock chance" updateCallback={updateVariantChance}/>
            </div>
            <VariantCreaturesSlots variantId={variant.id}/>
        </div>
    }</>
}

export default BankVariantFocused;