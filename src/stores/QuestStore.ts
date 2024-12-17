import { create } from "zustand"

type Quest = {
    id: string,
    script_name: string,
    directory: string,
    name: string,
    desc: string,
}

type Actions = {
    set_script_name: (s: string) => void,
    set_name: (s: string) => void,
    set_desc: (s: string) => void,
    set_directory: (s: string) => void,
    set_id: (s: string) => void
}

export const useQuestStore = create<Quest & Actions>((set, get) => ({
    id: "",
    script_name: "",
    directory: "",
    name: "",
    desc: "",

    set_id(s) {
        set({ id: s })
    },

    set_script_name(s) {
        set({ script_name: s })
    },

    set_name(s) {
        set({ name: s })
    },

    set_desc(s) {
        set({ desc: s })
    },

    set_directory(s) {
        set({ directory: s })
    }
}))