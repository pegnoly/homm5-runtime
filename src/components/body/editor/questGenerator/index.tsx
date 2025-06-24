import styles from '../../styles.module.css';
import QuestGeneratorBody from './body';
import QuestGeneratorHeader from './header';

function QuestGeneratorLayout() {
    return (
    <div className={styles.editor_layout}>
        <QuestGeneratorHeader/>
        <QuestGeneratorBody/>
    </div>
    )
}

export default QuestGeneratorLayout;