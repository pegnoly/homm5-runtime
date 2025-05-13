import { invoke } from "@tauri-apps/api/core";
import { Button, List, Typography} from "antd";
import { useEffect, useState } from "react";
import { Link, Route, Routes } from "react-router";
import BankFocused, { BankModel } from "./Bank";

function BanksConfiguratorMain() {
    const [banks, setBanks] = useState<BankModel[]>([]);

    useEffect(() => {
        invoke<BankModel[]>("get_all_banks")
            .then((data) => setBanks(data));
    }, [])

    return ( 
        <Routes>
            <Route path="/" element={<BanksList banks={banks}/>}/>
            <Route path="/bank/:id" element={<BankFocused/>}/>
        </Routes>
    )
}

function BanksList(props: {banks: BankModel[]}) {
    return <>
        <List>{props.banks.map((b, i) => (
            <Link key={i} to={`bank/${b.id}`}>
                <List.Item>
                    <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 17}}>{b.name}</Typography.Text>
                </List.Item>
            </Link>
        ))}</List>
    </>
}

export function BanksConfiguratorGlobals() {
    return <>
        <Button onClick={() => invoke("generate_banks_script")}>Generate banks script</Button>
    </>
}

export default BanksConfiguratorMain;