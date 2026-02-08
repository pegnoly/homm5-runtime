import { useParams } from "react-router";
import styles from '../styles.module.css';
import { EditorState, useEditorStateActions } from "../../../stores/EditorStateStore";
import QuestGeneratorLayout from "./questGenerator";
import DialogGeneratorLayout from "./dialogGenerator";
import ReserveHeroesGeneratorLayout from "./reserveHeroesGenerator";
import BankGeneratorLayout from "./bankGenerator";
import FightGeneratorLayout from "./fightGenerator";
import HeroCreatorLayout from "./heroCreator";
import CreatureCopyCreatorLayout from "./creatureCopyCreator";
import { useEffect } from "react";
import CreatureEditorLayout from "./creatureEditor";
import ArtifactEditorLayout from "./artifactEditor";
import SpellEditorLayout from "./spellEditor";

function EditorMain() {
    const { state } = useParams();
    const actions = useEditorStateActions();

    useEffect(() => {
        actions.setEditorState(state as EditorState);
    }, [state]);

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
        case EditorState.HeroCreator:
            return <HeroCreatorLayout/>
        case EditorState.CreatureCopyCreator:
            return <CreatureCopyCreatorLayout/>
        case EditorState.CreatureEditor:
            return <CreatureEditorLayout/>
        case EditorState.ArtifactEditor:
            return <ArtifactEditorLayout/>
        case EditorState.SpellEditor:
            return <SpellEditorLayout/>
        default:
            break;
    }
}

export default EditorMain;