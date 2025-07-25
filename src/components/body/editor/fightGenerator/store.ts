import { UUID } from "crypto"
import { create } from "zustand"

type Actions = {
    setCurrentAssetId: (value: UUID) => void,
    setCurrentAssetName: (value: string) => void,

    unloadAsset: () => void
}

type Store = {
    assetId: UUID | undefined,
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
        unloadAsset() {
            set({
                assetName: undefined,
                assetId: undefined
            })
        },
    }
}));

export const useFightAssetActions = () => fightAssetStore(state => state.actions);
export const useFightAssetName = () => fightAssetStore(state => state.assetName);
export const useFightAssetId = () => fightAssetStore(state => state.assetId);