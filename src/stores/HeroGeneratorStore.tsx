import { create } from "zustand"

type Data = {
    currentAssetId: number | null
}

type Action = {
    setCurrentAssetId: (value: number) => void
}

export const useHeroGeneratorStore = create<Data & Action>((set) => ({
    currentAssetId: null,
    setCurrentAssetId(value) {
        set({currentAssetId: value})
    },
}))