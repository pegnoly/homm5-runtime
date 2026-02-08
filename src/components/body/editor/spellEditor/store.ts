import { SpellModel } from "@/stores/GameDataStore"
import { create } from "zustand"

type Actions = {
    loadCurrent: (value: SpellModel) => void,
    updateCurrent: (value: SpellModel) => void
}

type Store = {
    current: SpellModel | undefined,
    actions: Actions
}

const store = create<Store>((set) => ({
    current: undefined,
    actions: {
        loadCurrent(value) {
            set({current: value});
        },

        updateCurrent(value) {
            set({current: value});
        },
    }
}));

export namespace SpellEditorStore {
    export const useCurrent = () => store(state => state.current);
    export const useActions = () => store(state => state.actions);
}