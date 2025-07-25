import { create } from "zustand"
import { BankVariant } from "../../types"

type Actions = {
    loadVariants: (values: BankVariant[]) => void
}

type Store = {
    variants: BankVariant[],

    currentId: number | undefined,
    actions: Actions
}

const store = create<Store>((set) => ({
    variants: [],
    currentId: undefined,

    actions: {
        loadVariants(values) {
            set({variants: values});
        },
    }
}));

export namespace BankVariantStore {
    export const useId = () => store(state => state.currentId);
    export const useVariants = () => store(state => state.variants);

    export const useActions = () => store(state => state.actions);
}