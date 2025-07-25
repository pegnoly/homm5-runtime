import { Card, SimpleGrid } from "@mantine/core";
import { BankDifficultyType, BankVariant } from "../types";
import { useQuery } from "@tanstack/react-query";
import { BankGeneratorApi } from "../api";
import { BankMainStore } from "../store";
import { BankDifficultyStore } from "./difficulty/store";
import { useEffect, useState } from "react";
import { Link, Route, Routes } from "react-router";
import BankVariantFocused from "./variant";
import { EditorState } from "@/stores/EditorStateStore";
import BankVariantCreator from "./variantCreator";

function BankVariantsPanel() {
    const bankId = BankMainStore.useId();

    return (
    <>
        {
            bankId == undefined ? null :
            <>
                <PanelRenderer/>
            </>
        }   
    </>
    )
}

function PanelRenderer() {
    return (
    <Routes>
        <Route path="/" element={<PanelGrid/>}/>
        <Route path="/variant/:id" element={<BankVariantFocused/>}/>
    </Routes>
    )
}

function PanelGrid() {
    const bankId = BankMainStore.useId();
    const [variants, setVariants] = useState<BankVariant[]>([]);

    async function variantCreated(value: BankVariant) {
        setVariants([...variants, value]);
    }

    return (
    <>
        <BankVariantCreator onCreated={variantCreated}/>
        <SimpleGrid cols={{lg: 3, sm: 2}}>{variants.map((v, i) => (
            <Link to={`/editor/${EditorState.Banks}/focused/${bankId}/variant/${v.id}`} key={i}>
                <Card radius={0}>
                    {v.label}
                </Card>
            </Link>
        ))}</SimpleGrid>
        <VariantsLoader onLoad={setVariants}/>
    </>
    
    )
}

function useVariants(bankId: number, difficulty: BankDifficultyType) {
    return useQuery({
        queryKey: ["bank_variants", bankId, difficulty],
        queryFn: async() => {
            return BankGeneratorApi.loadVariants(bankId, difficulty);
        }
    })
}

function VariantsLoader({onLoad}: {onLoad: (values: BankVariant[]) => void}) {
    const bankId = BankMainStore.useId();
    const difficulty = BankDifficultyStore.useType();

    const { data } = useVariants(bankId!, difficulty!);
    
    useEffect(() => {
        if (data != undefined && data != null) {
            onLoad(data);
        }
    }, [data])

    return null;
}

export default BankVariantsPanel;