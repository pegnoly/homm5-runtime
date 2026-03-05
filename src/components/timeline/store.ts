import { create } from "zustand"

type Actions = {
    changeActivity: (value: boolean) => void,
    addItem: (value: string) => void
}

type Store = {
    active: boolean,
    items: string[],
    actions: Actions
}

const store = create<Store>((set, get) => ({
    items: [],
    active: false,
    actions: {
        addItem(value) {
            let updatedItems = get().items
            if (get().items.length == 10) {
                updatedItems.shift();
            }
            set({items: [...updatedItems, value]})
        },
        changeActivity(value) {
            set({active: value});
        },
    }
}))

export namespace EditorTimelineStore {
    export const useItems = () => store(state => state.items);
    export const useActivity = () => store(state => state.active);
    export const useActions = () => store(state => state.actions);
}