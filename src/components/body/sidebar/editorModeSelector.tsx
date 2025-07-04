import { List } from '@mantine/core';
import { EditorState } from '../../../stores/EditorStateStore';
import styles from '../styles.module.css';
import EditorModeLink from './editorModeLink';

function EditorModeSelector() {

    return (
    <div className={styles.mode_selector}>
        <List className={styles.modes_list}>
            <EditorModeLink key={EditorState.Dialog} label='Dialogs' state={EditorState.Dialog}/>
            <EditorModeLink key={EditorState.Quest} label='Quests' state={EditorState.Quest}/>
            <EditorModeLink key={EditorState.Banks} label='Banks' state={EditorState.Banks}/>
            <EditorModeLink key={EditorState.ReserveHeroes} label='ReserveHeroes' state={EditorState.ReserveHeroes}/>
            <EditorModeLink key={EditorState.FightGenerator} label='FightGenerator' state={EditorState.FightGenerator}/>
            <EditorModeLink key={EditorState.HeroCreator} label='HeroCreator' state={EditorState.HeroCreator}/>
            <EditorModeLink key={EditorState.CreatureCopyCreator} label='CreatureCreator' state={EditorState.CreatureCopyCreator}/>
        </List>
    </div>
    )
}

export default EditorModeSelector;