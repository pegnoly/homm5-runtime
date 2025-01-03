import { invoke } from "@tauri-apps/api/core"
import { create } from "zustand"
import { Speaker } from "./SpeakersStore"

type Data = {
    current_dialog_id: string | null,
    current_dialog_name: string | null,
    current_dialog_script_name: string | null,
    current_dialog_speakers: Speaker[],
    current_dialog_labels: string[],
    current_step: number,
    current_label: string,
    current_variant_id: string | null,
    current_variant_speaker: string | null,
    current_variant_text: string
}

type Action = {
    set_current_dialog_id: (id: string) => void,
    set_current_dialog_name: (name: string) => void,
    set_current_dialog_script_name: (script_name: string) => void,
    set_current_dialog_speakers: (speakers: Speaker[]) => void,
    set_current_dialog_labels: (labels: string[]) => void,
    add_label: (label: string) => void,
    set_current_step: (step: number) => void,
    set_current_label: (label: string) => void,
    set_current_variant_id: (id: string) => void
    set_current_variant_speaker: (id: string | null) => void,
    set_current_variant_text: (text: string) => void
}

export const useCurrentDialogStore = create<Data & Action>((set, get) => ({
    current_dialog_id: null,
    current_dialog_name: null,
    current_dialog_script_name: null,
    current_dialog_speakers: [],
    current_dialog_labels: [],
    current_step: 0,
    current_label: "main",
    current_variant_id: null,
    current_variant_speaker: null,
    current_variant_text: "",

    set_current_dialog_id(id) {
        set({current_dialog_id: id})
    },

    set_current_dialog_name(name) {
        set({current_dialog_name: name})
    },

    set_current_dialog_script_name(script_name) {
        set({current_dialog_script_name: script_name})
    },

    set_current_dialog_speakers(speakers) {
        set({current_dialog_speakers: speakers})
    },

    set_current_dialog_labels(labels) {
        set({current_dialog_labels: labels})
    },

    async add_label(label) {
        const updatedLabels = [...get().current_dialog_labels, label]
        await invoke("update_dialog_labels", {dialogId: get().current_dialog_id, labels: updatedLabels})
        set({current_dialog_labels: updatedLabels})
    },

    set_current_step(step) {
        set({current_step: step})
    },

    set_current_label(label) {
        set({current_label: label})
    },

    set_current_variant_id(id) {
        set({current_variant_id: id})
    },

    set_current_variant_speaker(id) {
        set({current_variant_speaker: id})
    },

    set_current_variant_text(text) {
        set({current_variant_text: text})
    },
}))