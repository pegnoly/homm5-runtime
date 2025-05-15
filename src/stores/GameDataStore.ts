import { create } from "zustand"
import { ArtifactSlotType } from "../components/hero_generator/artsConfigurator/types"
import { TownType } from "../components/hero_generator/stacksConfigurator/main"

export type ArtifactModel = {
    id: number,
    name: string,
    slot: ArtifactSlotType
}

export type CreatureModel = {
    id: number,
    attack: number,
    defence: number,
    speed: number,
    initiative: number,
    exp: number,
    power: number,
    tier: number,
    town: TownType,
    grow: number,
    is_generatable: boolean,
    name: string,

}

type Data = {
    artifacts: ArtifactModel[],
    creatures: CreatureModel[]
}

type Action = {
    load_artifacts: (values: ArtifactModel[]) => void
    load_creatures: (values: CreatureModel[]) => void
}

const useGameDataStore = create<Data & Action>((set) => ({
    artifacts: [],
    creatures: [],

    load_artifacts(values) {
        set({artifacts: values});
    },
    load_creatures(values) {
        set({creatures: values});
    },
}))

export default useGameDataStore;