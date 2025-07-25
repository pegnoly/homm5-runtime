import { create } from "zustand"
import { BankFullModel } from "./types"
import { UUID } from "crypto"

type Actions = {
    loadBank: (value: BankFullModel) => void,
    updateRechargesCount: (value: number) => void,
    updateRechargeTimer: (value: number) => void,
    updateLuckLoss: (value: number) => void,
    updateMoraleLoss: (value: number) => void
}

type Store = {
    bankId: number | undefined,
    name: string | undefined,
    rechargeCount: number | undefined,
    rechargeTimer: number | undefined,
    luckLoss: number | undefined,
    moraleLoss: number | undefined,

    actions: Actions
}

const store = create<Store>((set) => ({
    bankId: undefined,
    name: undefined,
    rechargeCount: undefined,
    rechargeTimer: undefined,
    luckLoss: undefined,
    moraleLoss: undefined,
    
    actions: {
        loadBank(value) {
            set({
                bankId: value.id,
                name: value.name,
                rechargeCount: value.recharge_count,
                rechargeTimer: value.recharge_timer,
                luckLoss: value.luck_loss,
                moraleLoss: value.morale_loss
            });
        },
        updateRechargesCount(value) {
            set({rechargeCount: value});
        },
        updateRechargeTimer(value) {
            set({rechargeTimer: value});
        },
        updateLuckLoss(value) {
            set({luckLoss: value});
        },
        updateMoraleLoss(value) {
            set({moraleLoss: value});
        },
    }
}));

export namespace BankMainStore {
    export const useId = () => store(state => state.bankId);
    export const useName = () => store(state => state.name);
    export const useRechargeCount = () => store(state => state.rechargeCount);
    export const useRechargeTimer = () => store(state => state.rechargeTimer);
    export const useLuckLoss = () => store(state => state.luckLoss);
    export const useMoraleLoss = () => store(state => state.moraleLoss);

    export const useActions = () => store(state => state.actions);
}