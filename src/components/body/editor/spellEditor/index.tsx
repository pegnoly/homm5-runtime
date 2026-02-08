import styles from '../../styles.module.css';
import SpellEditorBody from './body';
import SpellEditorHeader from './header';

function SpellEditorLayout() {

    return <div className={styles.editor_layout}>
        <SpellEditorHeader/>
        <SpellEditorBody/>
    </div>
}

export default SpellEditorLayout;