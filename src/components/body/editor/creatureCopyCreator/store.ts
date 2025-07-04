import { create } from "zustand"
import { CreatableCreatureModel } from "./types"


type Actions = {
    initializeSession: (value: number) => void,
    setCurrentId: (value: number) => void,
    updateCreatedIds: (value: number []) => void,
    updateSelectedAbilities: (value: string []) => void,
    addModel: (value: number) => void,
    removeModel: (value: number) => void,
    updateModel: (value: CreatableCreatureModel) => void
}

type Store = {
    currentId: number | undefined,
    createdIds: number [],
    selectedAbilities: string [],
    models: CreatableCreatureModel [],

    actions: Actions
}

const store = create<Store>((set, get) => ({
    currentId: undefined,
    createdIds: [],
    selectedAbilities: [],
    models: [],

    actions: {
        initializeSession(value) {
            set({
                currentId: value,
                createdIds: [],
                selectedAbilities: [],
                models: []
            })
        },
        setCurrentId(value) {
            set({currentId: value});
        },
        updateCreatedIds(value) {
            set({createdIds: value});
        },
        updateSelectedAbilities(value) {
            set({selectedAbilities: value});
        },
        addModel(value) {
            set({models: [...get().models, {id: value, baseCreature: undefined, parentCreature: undefined, upgrades: [], innerName: undefined}]});
            set({currentId: value});
            set({createdIds: [...get().createdIds, value]})
        },
        removeModel(value) {
            set({models: get().models.filter(m => m.id !=  value)});
        },
        updateModel(value) {
            set({models: get().models.map(m => {
                if (m.id == value.id) {
                    m = value;
                    return m;
                } else {
                    return m;
                }
            })})
        },
    }
}));


namespace CreatureCopyCreator {
    export const useCurrentId = () => store(state => state.currentId)
    export const useIds = () => store(state => state.createdIds);
    export const useAbilities = () => store(state => state.selectedAbilities);
    export const useActions = () => store(state => state.actions); 
    export const useModels = () => store(state => state.models);
}

export default CreatureCopyCreator;