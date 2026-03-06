import {EditorState, useEditorState} from '@/stores/EditorStateStore.ts';
import BankGeneratorGlobals from '../editor/bankGenerator/globals';
import DialogGeneratorGlobals from '../editor/dialogGenerator/globals';
import FightGeneratorGlobals from '../editor/fightGenerator/globals';
import QuestGeneratorGlobals from '../editor/questGenerator/globals';
import ReserveHeroesGeneratorGlobals from '../editor/reserveHeroesGenerator/globals';
import styles from '../styles.module.css';
import CreatureEditorGlobals from "@/components/body/editor/creatureEditor/globals.tsx";

function EditorGlobals() {

    return (
    <div className={styles.editor_globals}>
        <RenderEditorGlobals/>
    </div>
    )
}

function RenderEditorGlobals() {
    const editorState = useEditorState();

    switch (editorState) {
        case EditorState.FightGenerator:
            return <FightGeneratorGlobals/> 
        case EditorState.Dialog:
            return <DialogGeneratorGlobals/>
        case EditorState.Quest: 
            return <QuestGeneratorGlobals/>
        case EditorState.ReserveHeroes: 
            return <ReserveHeroesGeneratorGlobals/>
        case EditorState.Banks:
            return <BankGeneratorGlobals/>
        case EditorState.CreatureEditor:
            return <CreatureEditorGlobals/>
        default:
            return null;
    }
}

export default EditorGlobals;