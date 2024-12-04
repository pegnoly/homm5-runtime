import { create } from "zustand"

type Quest = {
    script_name: string,
    directory: string,
    name: string,
    desc: string,
    progresses: string[]
}

type Actions = {
    get_progress: (n: number) => string,
    set_script_name: (s: string) => void,
    set_name: (s: string) => void,
    set_desc: (s: string) => void,
    set_directory: (s: string) => void,
}

export const useQuestStore = create<Quest & Actions>((set, get) => ({
    script_name: "",
    directory: "",
    name: "",
    desc: "",
    progresses: [],

    get_progress(n) {
        return get().progresses[n]
    },

    set_script_name(s) {
        set({script_name: s})
    },

    set_name(s) {
        set({name: s})
    },

    set_desc(s) {
        set({desc: s})
    },

    set_directory(s) {
        set({directory: s})
    }
}))