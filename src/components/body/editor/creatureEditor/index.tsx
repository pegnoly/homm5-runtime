import styles from '../../styles.module.css';
import CreatureEditorBody from './body';
import CreatureEditorHeader from './header';

function CreatureEditorLayout() {

    return (
        <div className={styles.editor_layout}>
            <CreatureEditorHeader/>
            <CreatureEditorBody/>
        </div>
    )
}

export default CreatureEditorLayout;