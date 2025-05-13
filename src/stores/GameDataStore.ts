import { create } from "zustand"
import { ArtifactSlotType } from "../components/hero_generator/artsConfigurator/types"

export type ArtifactModel = {
    id: number,
    name: string,
    slot: ArtifactSlotType
}

type Data = {
    artifacts: ArtifactModel[]
}

type Action = {
    load_artifacts: (values: ArtifactModel[]) => void
}

const useGameDataStore = create<Data & Action>((set) => ({
    artifacts: [],
    load_artifacts(values) {
        set({artifacts: values})
    },
}))

export default useGameDataStore;