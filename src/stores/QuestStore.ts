import { create } from "zustand"

type Quest = {
    id: string | null,
    script_name: string,
    directory: string,
    name: string,
    desc: string,
    secondary: boolean,
    active: boolean
}

type Actions = {
    set_script_name: (s: string) => void,
    set_name: (s: string) => void,
    set_desc: (s: string) => void,
    set_directory: (s: string) => void,
    set_id: (s: string) => void,
    set_secondary: (b: boolean) => void,
    set_active: (b: boolean) => void
}

export const useCurrentQuestStore = create<Quest & Actions>((set) => ({
    id: null,
    script_name: "",
    directory: "",
    name: "",
    desc: "",
    secondary: false,
    active: false,

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
    },

    set_secondary(b) {
        set({ secondary: b })
    },

    set_active(b) {
        set({ active: b })
    },
}))