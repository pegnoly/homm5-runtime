import { invoke } from "@tauri-apps/api/core";
import { BankDifficulty, BankDifficultyType, BankFullModel, BankLoadingModel, BankVariant } from "./types";

export class BankGeneratorApi {
    static async loadBanks(): Promise<BankLoadingModel[]> {
        return invoke<BankLoadingModel[]>("get_all_banks");
    }

    static async loadBank(id: number): Promise<BankFullModel | null> {
        return invoke<BankFullModel | null>("load_bank", {id: id});
    }

    static async updateRechargesCount(id: number, count: string | number): Promise<number> {
        return invoke("update_bank_recharges_count", {id: id, count: count});
    }

    static async updateRechargeTimer(id: number, timer: string | number): Promise<number> {
        return invoke("update_bank_recharge_timer", {id: id, timer: timer});
    }

    static async updateLuckLoss(id: number, loss: string | number): Promise<number> {
        return invoke("update_bank_morale_loss", {id: id, loss: loss});
    }

    static async updateMoraleLoss(id: number, loss: string | number): Promise<number> {
        return invoke("update_bank_luck_loss", {id: id, loss: loss});
    }

    static async loadDifficulty(id: number, type: BankDifficultyType): Promise<BankDifficulty | null> {
        return invoke<BankDifficulty | null>("load_difficulty", {bankId: id, difficulty: type});
    }

    static async updateChance(id: number, chance: string | number): Promise<number> {
        return invoke<number>("update_bank_difficulty_chance", {id: id, chance: chance});
    }

    static async loadVariants(id: number, difficulty: BankDifficultyType): Promise<BankVariant[]> {
        return invoke<BankVariant[]>("load_bank_variants", {bankId: id, difficulty: difficulty});
    }

    static async createVariant(bankId: number, label: string, difficulty: BankDifficultyType): Promise<BankVariant> {
        return invoke<BankVariant>("create_bank_variant", {bankId: bankId, label: label, difficulty: difficulty});
    }
}