import { create } from "zustand"
import { CreatureEditableModel } from "./types"

type Actions = {
    loadCreature: (value: CreatureEditableModel) => void,
    updateCreature: (newValue: CreatureEditableModel) => void
}

type Store = {
    current: CreatureEditableModel | undefined,

    actions: Actions
}

const store = create<Store>((set) => ({
    current: undefined,

    actions: {
        loadCreature(value) {
            set({ current: value });
        },
        updateCreature(newValue) {
            set({ current: newValue });
        }
    }
}))

export namespace CreatureEditorStore {
    export const useCurrent = () => store(state => state.current);
    export const useActions = () => store(state => state.actions);
}