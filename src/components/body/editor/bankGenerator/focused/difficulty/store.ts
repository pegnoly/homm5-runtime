import { create } from "zustand"
import { BankDifficulty, BankDifficultyType } from "../../types"

type Actions = {
    loadDifficulty: (value: BankDifficulty) => void,
    updateType: (value: BankDifficultyType) => void,
    updateChance: (value: number) => void
}

type Store = {
    id: number | undefined,
    type: BankDifficultyType | undefined,
    chance: number | undefined,

    actions: Actions
}

const store = create<Store>((set) => ({
    id: undefined,
    type: BankDifficultyType.Easy,
    chance: undefined,

    actions: {
        loadDifficulty(value) {
            set({id: value.id, chance: value.chance});
        },
        updateType(value) {
            set({type: value});
        },
        updateChance(value) {
            set({chance: value});
        },
    }
}));

export namespace BankDifficultyStore {
    export const useId = () => store(state => state.id);
    export const useType = () => store(state => state.type);
    export const useChance = () => store(state => state.chance);

    export const useActions = () => store(state => state.actions);
}