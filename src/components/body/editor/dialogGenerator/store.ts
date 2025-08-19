import { create } from "zustand"
import { Dialog, DialogSimple, DialogVariant, Speaker } from "./types"

type Actions = {
    loadDialogs: (value: DialogSimple []) => void,
    loadSpeakers: (value: Speaker []) => void,

    loadCurrentDialog: (value: Dialog) => void,
    setCurrentDialogId: (value: number) => void,
    updateDialogLabels: (value: string []) => void,

    loadVariant: (value: DialogVariant) => void,
    setCurrentVariantStep: (value: number) => void,
    setCurrentVariantLabel: (value: string) => void,
    setCurrentVariantSpeaker: (value: number) => void,
    setCurrentVariantText: (value: string) => void,

    setCurrentVariantSaved: (value: boolean) => void
}

type Store = {
    dialogs: DialogSimple[] | undefined,
    speakers: Speaker [] | undefined,

    currentDialogId: number | undefined,
    currentDialogName: string | undefined,
    currentDialogScriptName: string | undefined,
    currentDialogDirectory: string | undefined,
    currentDialogLabels: string [] | undefined,
    currentDialogSpeakers: number [] | undefined

    currentVariantId: number | undefined,
    currentVariantStep: number | undefined,
    currentVariantLabel: string | undefined,
    currentVariantSpeaker: number | null,
    currentVariantText: string | undefined,

    currentVariantSaved: boolean,

    actions: Actions
}

const dialogGeneratorStore = create<Store>((set) => ({
    dialogs: undefined,
    speakers: undefined,

    currentDialogId: undefined,
    currentDialogName: undefined,
    currentDialogScriptName: undefined,
    currentDialogDirectory: undefined,
    currentDialogLabels: undefined,
    currentDialogSpeakers: undefined,

    currentVariantId: undefined,
    currentVariantStep: undefined,
    currentVariantLabel: undefined,
    currentVariantSpeaker: null,
    currentVariantText: undefined,

    currentVariantSaved: true,

    actions: {
        loadDialogs(value) {
            set({dialogs: value});
        },
        loadSpeakers(value) {
            set({speakers: value});
        },
        loadCurrentDialog(value) {
            set({
                currentDialogId: value.id,
                currentDialogName: value.name,
                currentDialogScriptName: value.script_name,
                currentDialogDirectory: value.directory,
                currentDialogLabels: value.labels.labels,
                currentDialogSpeakers: value.speakers_ids.ids
            })
        },
        loadVariant(value) {
            set({
                currentVariantId: value.id,
                currentVariantStep: value.step,
                currentVariantSpeaker: value.speaker_id,
                currentVariantLabel: value.label,
                currentVariantText: value.text
            });
        },
        setCurrentDialogId(value) {
            set({currentDialogId: value});
        },
        updateDialogLabels(value) {
            set({currentDialogLabels: value});
        },
        setCurrentVariantStep(value) {
            set({currentVariantStep: value})
        },
        setCurrentVariantLabel(value) {
            set({currentVariantLabel: value});
        },
        setCurrentVariantSpeaker(value) {
            set({currentVariantSpeaker: value});
        },
        setCurrentVariantText(value) {
            set({currentVariantText: value});
        },
        setCurrentVariantSaved(value) {
            set({currentVariantSaved: value});
        },
    }
}));

export const useDialogActions = () => dialogGeneratorStore(state => state.actions);
export const useDialogs = () => dialogGeneratorStore(state => state.dialogs);
export const useSpeakers = () => dialogGeneratorStore(state => state.speakers);
export const useCurrentDialogId = () => dialogGeneratorStore(state => state.currentDialogId);
export const useCurrentDialogVariantId = () => dialogGeneratorStore(state => state.currentVariantId);
export const useCurrentDialogVariantLabel = () => dialogGeneratorStore(state => state.currentVariantLabel);
export const useCurrentDialogVariantStep = () => dialogGeneratorStore(state => state.currentVariantStep);
export const useCurrentDialogVariantSpeaker = () => dialogGeneratorStore(state => state.currentVariantSpeaker);
export const useCurrentDialogVariantText = () => dialogGeneratorStore(state => state.currentVariantText);
export const useDialogLabels = () => dialogGeneratorStore(state => state.currentDialogLabels);
export const useDialogSpeakers = () => dialogGeneratorStore(state => state.currentDialogSpeakers);
export const useCurrentVariantSaved = () => dialogGeneratorStore(state => state.currentVariantSaved)