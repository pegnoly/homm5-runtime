import { useQuery } from "@tanstack/react-query";
import { BankGeneratorApi } from "../api";
import { BankMainStore } from "../store";
import { useParams } from "react-router";
import BankProps from "./props";
import { useEffect } from "react";
import BankDifficultyInfo from "./difficulty";
import BankVariantsPanel from "./variantsPanel";

function BankFocused() {
    const { id } = useParams();

    return (
    <>
        <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
            <div style={{width: '15%', height: '100%'}}>
                <div style={{width: '100%', height: '100%', backgroundColor: 'Scrollbar'}}>
                    <div style={{width: '100%', height: '40%'}}>
                        <BankProps/>
                    </div>
                    <div style={{width: '100%', height: '60%'}}>
                        <BankDifficultyInfo/>
                    </div>
                </div>
            </div>
            <div style={{width: '85%', height: '100%'}}>
                <div style={{width: '100%', height: '100%'}}> 
                    <BankVariantsPanel/>
                </div>
            </div>
        </div>
        <BankLoader id={parseInt(id!)}/>
    </>
    )
}

function useBank(id: number) {
    return useQuery({
        queryKey: ["bank", id],
        queryFn: async() => {
            return BankGeneratorApi.loadBank(id);
        }
    })
}

function BankLoader({id}: {id: number}) {
    const actions = BankMainStore.useActions();
    const { data } = useBank(id);

    useEffect(() => {
        if (data != undefined && data != null) {
            actions.loadBank(data);
        }
    }, [data]);

    return null;
}

export default BankFocused;