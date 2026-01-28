import { ArtifactModel } from "@/stores/GameDataStore"
import { create } from "zustand"

type Actions = {
    loadCurrent: (model: ArtifactModel) => void
    updateCurrent: (model: ArtifactModel) => void
}

type Store = {
    current: ArtifactModel | undefined,
    actions: Actions
}

const store = create<Store>((set) => ({
    current: undefined,
    actions: {
        loadCurrent(model) {
            set({current: model});
        },
        updateCurrent(model) {
            set({current: model});
        },
    }
}));

export namespace ArtifactEditorStore {
    export const useCurrent = () => store(state => state.current);
    export const useActions = () => store(state => state.actions);
}