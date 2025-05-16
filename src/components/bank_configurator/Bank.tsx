import { invoke } from "@tauri-apps/api/core";
import { Col, Typography } from "antd";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import BankVariants from "./BankVariants";
import BankProps from "./BankProps";
import BankDifficulties from "./BankDifficulties";

enum BankType {
    Crypt = "BTD_BANK_CRYPT",
    Pyramid = "BTD_BANK_PYRAMID",
    MagiVault = "BTD_BANK_MAGI_VAULT",
    DragonUtopia = "BTD_BANK_DRAGON_UTOPIA",
    ElementalStockpile = "BTD_BANK_ELEMENTAL_STOCKPILE",
    DwarvenTreasure = "BTD_BANK_DWARVEN_TREASURE",
    BloodTemple = "BTD_BANK_BLOOD_TEMPLE",
    TreantThicket = "BTD_BANK_TREANT_THICKET",
    GargoyleStonevault = "BTD_BANK_GARGOYLE_STONEVAULT",
    SunkenTemple = "BTD_BANK_SUNKEN_TEMPLE"
}

export type BankModel = {
    id: number,
    type: BankType,
    name: string,
    recharge_timer: number,
    recharge_count: number,
    luck_loss: number,
    morale_loss: number
}

function BankFocused() {
    const { id } = useParams();

    const [bank, setBank] = useState<BankModel | null>(null);

    useEffect(() => {
        if (id != undefined) {
            invoke<BankModel>("load_bank", {id: parseInt(id)})
                .then((data) => setBank(data));
        }
    }, [id]);

    return <div style={{paddingLeft: '5%'}}>
        <div style={{justifyContent: 'center', display: 'flex'}}>
            <Typography.Text style={{textAlign: 'center', fontFamily: 'cursive', fontWeight: 'bold', fontSize: 20}}>{bank?.name}</Typography.Text>
        </div>
        <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
            <Col span={11}>
                <div style={{display: 'flex', flexDirection: 'column', gap: 50, height: '100%'}}>
                    <BankProps bank={bank}/>
                    <BankDifficulties bankId={bank?.id}/>
                </div>
            </Col>
            <Col span={11}><BankVariants bankId={bank?.id}/></Col>
        </div>
    </div>
}

export default BankFocused;