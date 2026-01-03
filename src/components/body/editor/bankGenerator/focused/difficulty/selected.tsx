import { useMutation, useQuery } from "@tanstack/react-query";
import { BankMainStore } from "../../store";
import { BankDifficultyType } from "../../types";
import { BankDifficultyStore } from "./store";
import { BankGeneratorApi } from "../../api";
import { useEffect } from "react";
import EditableProperty from "@/components/common/editableProperty";

function BankDifficultySelected() {
    const bankId = BankMainStore.useId();
    const difficultyId = BankDifficultyStore.useId();
    const chance = BankDifficultyStore.useChance();
    const actions = BankDifficultyStore.useActions();

    const mutation = useMutation({
        mutationFn: async(data: {id: number, value: string | number}) => {
            return BankGeneratorApi.updateChance(data.id, data.value);
        },
        onSuccess(data, _variables, _context) {
            actions.updateChance(data);
        },
    })

    return (
    <>
        {
            bankId == undefined  ? null :
            <>
                <EditableProperty 
                    size="xs"
                    label="Difficuly chance"
                    initialValue={chance?.toString()!}
                    onSave={(value) => mutation.mutate({id: difficultyId!, value: value})}
                />
                <BankDifficultyLoader/>  
            </>
        }      
    </>

    )
}

function useDifficulty(id: number, type: BankDifficultyType) {
    return useQuery({
        queryKey: ["difficulty", id, type],
        queryFn: async() => {
            return BankGeneratorApi.loadDifficulty(id, type);
        }
    })
}

function BankDifficultyLoader() {
    const bankId = BankMainStore.useId();
    const type = BankDifficultyStore.useType();
    const actions = BankDifficultyStore.useActions();

    const { data } = useDifficulty(bankId!, type!);

    useEffect(() => {
        if (data != undefined && data != null) {
            actions.loadDifficulty(data);
        }
    }, [data])

    return null;
}

export default BankDifficultySelected;