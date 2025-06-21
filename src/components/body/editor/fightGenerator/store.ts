import { create } from "zustand"

type Actions = {
    setCurrentAssetId: (value: number) => void,
    setCurrentAssetName: (value: string) => void
}

type Store = {
    assetId: number | undefined,
    assetName: string | undefined,

    actions: Actions
}

const fightAssetStore = create<Store>((set) => ({
    assetId: undefined,
    assetName: undefined,

    actions: {
        setCurrentAssetId(value) {
            set({assetId: value})
        },
        setCurrentAssetName(value) {
            set({assetName: value})
        },
    }
}));

export const useFightAssetActions = () => fightAssetStore(state => state.actions);
export const useFightAssetName = () => fightAssetStore(state => state.assetName);
export const useFightAssetId = () => fightAssetStore(state => state.assetId);