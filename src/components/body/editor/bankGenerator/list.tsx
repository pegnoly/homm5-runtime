import { useQuery } from "@tanstack/react-query";
import { BankGeneratorApi } from "./api";
import { useEffect, useState } from "react";
import { BankLoadingModel } from "./types";
import { List } from "@mantine/core";
import { Link } from "react-router";
import { EditorState } from "@/stores/EditorStateStore";

function BanksList() {
    const [banks, setBanks] = useState<BankLoadingModel[]>([]);

    return (
    <>
        <BanksLoader onLoad={setBanks}/>
        <BanksListRenderer banks={banks}/>
    </>
    )
}

function BanksListRenderer({banks}: {banks: BankLoadingModel[]}) {
    return (
    <List>{banks.map((b, i) => (
        <Link to={`/editor/${EditorState.Banks}/focused/${b.id}`} key={i}>
            <div>
                {b.name}
            </div>
        </Link>
    ))}</List>
    )
}

function useBanks() {
    return useQuery({
        queryKey: ["all_banks"],
        queryFn: async() => {
            return BankGeneratorApi.loadBanks();
        }
    })
}

function BanksLoader({onLoad}: {onLoad: (values: BankLoadingModel[]) => void}) {
    const { data } = useBanks();

    useEffect(() => {
        if (data != undefined) {
            onLoad(data);
        }
    }, [data])

    return null
}

export default BanksList;