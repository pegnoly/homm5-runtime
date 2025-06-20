import { create } from "zustand"

type Actions = {
    setCurrentMapId: (value: number) => void
}

type Store = {
    currentMapId: number | undefined,
    actions: Actions
}

const useCommonStore = create<Store>((set) => ({
    currentMapId: undefined,

    actions: {
        setCurrentMapId(value) {
            set({currentMapId: value});
        },
    }
}));

export const useCurrentMapId = () => useCommonStore((state) => state.currentMapId);
export const useCommonActions = () => useCommonStore((state) => state.actions);