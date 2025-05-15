import { create } from "zustand"
import { HeroAssetStackModel } from "../components/hero_generator/stacksConfigurator/main"

type Data = {
    currentAssetId: number | undefined,
    currentStack: number | undefined,
    currentStackAsset: HeroAssetStackModel | null
}

type Action = {
    setCurrentAssetId: (value: number) => void,
    setCurrentStack: (value: number) => void,
    setCurrentStackAsset: (value: HeroAssetStackModel) => void
}

const useHeroGeneratorStore = create<Data & Action>((set) => ({
    currentAssetId: undefined,
    currentStack: undefined,
    currentStackAsset: null,

    setCurrentAssetId(value) {
        set({currentAssetId: value});
    },
    setCurrentStack(value) {
        set({currentStack: value});
    },
    setCurrentStackAsset(value) {
        set({currentStackAsset: value});
    },
}));

export default useHeroGeneratorStore;