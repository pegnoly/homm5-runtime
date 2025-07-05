import { create } from "zustand"
import { CreatableCreatureModel } from "./types"


type Actions = {
    initializeSession: (startId: number, name: string) => void,
    setCurrentId: (value: number) => void,
    updateCreatedIds: (value: number []) => void,
    updateSelectedAbilities: (value: number []) => void,
    addModel: (value: number) => void,
    removeModel: (value: number) => void,
    updateModel: (value: CreatableCreatureModel) => void
}

type Store = {
    currentName: string | undefined,
    currentId: number | undefined,
    createdIds: number [],
    selectedAbilities: number [],
    models: CreatableCreatureModel [],

    actions: Actions
}

const store = create<Store>((set, get) => ({
    currentName: undefined,
    currentId: undefined,
    createdIds: [],
    selectedAbilities: [],
    models: [],

    actions: {
        initializeSession(startId, name) {
            set({
                currentId: startId,
                createdIds: [],
                selectedAbilities: [],
                models: [],
                currentName: name
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
            set({models: [...get().models, {id: value, base_creature: undefined, parent_creature: undefined, upgrades: [], inner_name: undefined}]});
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
    export const useName = () => store(state => state.currentName);
    export const useCurrentId = () => store(state => state.currentId)
    export const useIds = () => store(state => state.createdIds);
    export const useAbilities = () => store(state => state.selectedAbilities);
    export const useActions = () => store(state => state.actions); 
    export const useModels = () => store(state => state.models);
}

export default CreatureCopyCreator;