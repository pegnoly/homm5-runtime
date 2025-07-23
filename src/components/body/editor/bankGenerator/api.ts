import { invoke } from "@tauri-apps/api/core";
import { BankFullModel, BankLoadingModel } from "./types";

export class BankGeneratorApi {
    static async loadBanks(): Promise<BankLoadingModel[]> {
        return invoke<BankLoadingModel[]>("get_all_banks");
    }

    static async loadBank(id: number): Promise<BankFullModel | null> {
        return invoke<BankFullModel | null>("load_bank", {id: id});
    }

    static async updateRechargesCount(id: number, count: string): Promise<number> {
        return invoke("update_bank_recharges_count", {id: id, count: count});
    }

    static async updateRechargeTimer(id: number, timer: string): Promise<number> {
        return invoke("update_bank_recharge_timer", {id: id, timer: timer});
    }

    static async updateLuckLoss(id: number, loss: string): Promise<number> {
        return invoke("update_bank_morale_loss", {id: id, loss: loss});
    }

    static async updateMoraleLoss(id: number, loss: string): Promise<number> {
        return invoke("update_bank_luck_loss", {id: id, loss: loss});
    }
}