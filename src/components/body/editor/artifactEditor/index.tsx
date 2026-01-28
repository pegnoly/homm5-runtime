import styles from '../../styles.module.css';
import ArtifactEditorBody from './body';
import ArtifactEditorHeader from './header';

function ArtifactEditorLayout() {

    return <div className={styles.editor_layout}>
        <ArtifactEditorHeader/>
        <ArtifactEditorBody/>
    </div>
}

export default ArtifactEditorLayout;