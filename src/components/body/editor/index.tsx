import { useParams } from "react-router";
import styles from '../styles.module.css';
import { EditorState, useEditorStateActions } from "../../../stores/EditorStateStore";
import QuestGeneratorLayout from "./questGenerator";
import DialogGeneratorLayout from "./dialogGenerator";
import ReserveHeroesGeneratorLayout from "./reserveHeroesGenerator";
import BankGeneratorLayout from "./bankGenerator";
import FightGeneratorLayout from "./fightGenerator";

function EditorMain() {
    const { state } = useParams();
    const actions = useEditorStateActions();

    actions.setEditorState(state as EditorState);

    return (
    <div className={styles.editor}>
        <RenderEditorMode state={state as EditorState}/>
    </div>
    )
}

function RenderEditorMode(params: {
    state: EditorState
}) {
    switch (params.state) {
        case EditorState.Quest:
            return <QuestGeneratorLayout/>
        case EditorState.Dialog:
            return <DialogGeneratorLayout/>
        case EditorState.ReserveHeroes:
            return <ReserveHeroesGeneratorLayout/>
        case EditorState.Banks:
            return <BankGeneratorLayout/>
        case EditorState.FightGenerator:
            return <FightGeneratorLayout/>
        default:
            break;
    }
}

export default EditorMain;