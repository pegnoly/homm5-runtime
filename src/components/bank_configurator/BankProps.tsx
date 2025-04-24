import { useState } from "react";
import BankStringProperty from "./utils";
import { invoke } from "@tauri-apps/api/core";
import { BankModel } from "./Bank";
import { Typography } from "antd";

function BankProps(params: {bank: BankModel | null}) {
    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center'}}>
            <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 15, color: 'darkorchid'}}>Props</Typography.Text>
            {
                params.bank == null ? <h1>Bank undefined</h1> : 
                <>
                    <BankRechargesCount id={params.bank.id} initial={params.bank.recharge_count}/>
                    <BankRechargeTimer id={params.bank.id} initial={params.bank.recharge_timer}/>
                    <BankMoraleLoss id={params.bank.id} initial={params.bank.morale_loss}/>
                    <BankLuckLoss id={params.bank.id} initial={params.bank.luck_loss}/>
                </>
            }
        </div>
    )
}

function BankRechargesCount(params: {id: number, initial: number}) {
    const [count, setCount] = useState<number>(params.initial);

    async function updateRechargesCount(newCount: string) {
        await invoke<number>("update_bank_recharges_count", {bankId: params.id, count: newCount})
            .then((data) => setCount(data));
    }

    return <BankStringProperty initialValue={count} text="Bank recharges count" updateCallback={updateRechargesCount}/>
}

function BankRechargeTimer(params: {id: number, initial: number}) {
    const [timer, setTimer] = useState<number>(params.initial);

    async function updateRechargeTimer(newTimer: string) {
        await invoke<number>("update_bank_recharge_timer", {bankId: params.id, timer: newTimer})
            .then((data) => setTimer(data));
    }

    return <BankStringProperty initialValue={timer} text="Bank recharge timer" updateCallback={updateRechargeTimer}/>
}

function BankMoraleLoss(params: {id: number, initial: number}) {
    const [loss, setLoss] = useState<number>(params.initial);

    async function updateMoraleLoss(newLoss: string) {
        await invoke<number>("update_bank_morale_loss", {bankId: params.id, loss: newLoss})
            .then((data) => setLoss(data));
    }

    return <BankStringProperty initialValue={loss} text="Bank morale loss" updateCallback={updateMoraleLoss}/>
}

function BankLuckLoss(params: {id: number, initial: number}) {
    const [loss, setLoss] = useState<number>(params.initial);

    async function updateLuckLoss(newLoss: string) {
        await invoke<number>("update_bank_luck_loss", {bankId: params.id, loss: newLoss})
            .then((data) => setLoss(data));
    }

    return <BankStringProperty initialValue={loss} text="Bank luck loss" updateCallback={updateLuckLoss}/>
}

export default BankProps;