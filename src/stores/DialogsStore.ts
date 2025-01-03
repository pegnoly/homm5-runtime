import { create } from "zustand"

export type Dialog = {
    id: string,
    name: string
}

type Data = {
    dialogs: Dialog[]
}

type Actions = {
    set_dialogs: (ds: Dialog[]) => void,
    add_dialog: (d: Dialog) => void
}

export const useDialogsStore = create<Data & Actions>((set, get) => ({
    dialogs: [],

    set_dialogs(ds) {
        set({dialogs: ds})
    },
    
    add_dialog(d) {
        const updated_dialogs = [...get().dialogs, d]
        set({dialogs: updated_dialogs})
    },
}))