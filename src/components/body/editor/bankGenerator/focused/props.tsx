import EditableProperty from "@/components/common/editableProperty";
import { BankMainStore } from "../store";
import { useMutation } from "@tanstack/react-query";
import { BankGeneratorApi } from "../api";
import { Text } from "@mantine/core";

function BankProps() {
    const id = BankMainStore.useId();

    return (
    <>
        {
            id == undefined ? null :
            <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column', gap: '2%'}}>
                <BankName/>
                <BankRechargeCount/>
                <BankRechargeTimer/>
                <BankLuckLoss/>
                <BankMoraleLoss/>
            </div>
        }
    </>
    )
}

function BankName() {
    const name = BankMainStore.useName();

    return (
    <Text>{name}</Text>
    )
}

function BankRechargeCount() {
    const id = BankMainStore.useId();
    const count = BankMainStore.useRechargeCount();
    const actions = BankMainStore.useActions();

    const mutation = useMutation({
        mutationFn: async(data: {id: number, value: string | number}) => {
            return BankGeneratorApi.updateRechargesCount(data.id, data.value)
        },
        onSuccess(data, _variables, _context) {
            actions.updateRechargesCount(data);
        },
    })

    return (
    <>
        {
            id == undefined || count == undefined ? null :
            <EditableProperty 
                size="xs"
                label="Recharges count"
                initialValue={count.toString()}
                onSave={(value) => mutation.mutate({id: id, value: value})}
            />
        }
    </>
    )
}

function BankRechargeTimer() {
    const id = BankMainStore.useId();
    const timer = BankMainStore.useRechargeTimer();
    const actions = BankMainStore.useActions();

    const mutation = useMutation({
        mutationFn: async(data: {id: number, value: string | number}) => {
            return BankGeneratorApi.updateRechargeTimer(data.id, data.value)
        },
        onSuccess(data, _variables, _context) {
            actions.updateRechargeTimer(data);
        },
    })

    return (
    <>
        {
            id == undefined || timer == undefined ? null :
            <EditableProperty 
                size="xs"
                label="Recharge timer"
                initialValue={timer.toString()}
                onSave={(value) => mutation.mutate({id: id, value: value})}
            />
        }
    </>
    )
}

function BankLuckLoss() {
    const id = BankMainStore.useId();
    const loss = BankMainStore.useLuckLoss();
    const actions = BankMainStore.useActions();

    const mutation = useMutation({
        mutationFn: async(data: {id: number, value: string | number}) => {
            return BankGeneratorApi.updateLuckLoss(data.id, data.value)
        },
        onSuccess(data, _variables, _context) {
            actions.updateLuckLoss(data);
        },
    })

    return (
    <>
        {
            id == undefined || loss == undefined ? null :
            <EditableProperty 
                size="xs"
                label="Luck loss"
                initialValue={loss.toString()}
                onSave={(value) => mutation.mutate({id: id, value: value})}
            />
        }
    </>
    )
}

function BankMoraleLoss() {
    const id = BankMainStore.useId();
    const loss = BankMainStore.useMoraleLoss();
    const actions = BankMainStore.useActions();

    const mutation = useMutation({
        mutationFn: async(data: {id: number, value: string | number}) => {
            return BankGeneratorApi.updateMoraleLoss(data.id, data.value)
        },
        onSuccess(data, _variables, _context) {
            actions.updateMoraleLoss(data);
        },
    })

    return (
    <>
        {
            id == undefined || loss == undefined ? null :
            <EditableProperty 
                size="xs"
                label="Morale loss"
                initialValue={loss.toString()}
                onSave={(value) => mutation.mutate({id: id, value: value})}
            />
        }
    </>
    )
}

export default BankProps;