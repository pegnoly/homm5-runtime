import { create } from "zustand"

type Actions = {
    setCurrentAssetId: (value: number) => void,
    setCurrentAssetName: (value: string) => void,

    unloadAsset: () => void
}

type Store = {
    assetId: number | undefined,
    assetName: string | undefined,

    actions: Actions
}

const store = create<Store>((set) => ({
    assetId: undefined,
    assetName: undefined,

    actions: {
        setCurrentAssetId(value) {
            set({assetId: value})
        },
        setCurrentAssetName(value) {
            set({assetName: value})
        },
        unloadAsset() {
            set({
                assetName: undefined,
                assetId: undefined
            })
        },
    }
}));

export namespace FightAssetStore {
    export const useActions = () => store(state => state.actions);
    export const useAssetName = () => store(state => state.assetName);
    export const useAssetId = () => store(state => state.assetId);
}