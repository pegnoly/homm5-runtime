import { create } from "zustand"

export enum EditorState {
    Quest = "Quest",
    Dialog = "Dialog",
    ReserveHeroes = "ReserveHeroes",
    Banks = "Banks",
    FightGenerator = "FightGenerator",
    HeroCreator = "HeroCreator",
    CreatureCopyCreator = "CreatureCopyCreator",
    CreatureEditor = "CreatureEditor"
}

type Actions = {
    setEditorState: (state: EditorState) => void
}

type Store = {
    editorState: EditorState | undefined,
    actions: Actions
}

const useEditorStateStore = create<Store>((set) => ({
    editorState: undefined,
    actions: {
        setEditorState(state) {
            set({editorState: state})
        },
    }
}))

export const useEditorState = () => useEditorStateStore((state) => state.editorState);
export const useEditorStateActions = () => useEditorStateStore((state) => state.actions);