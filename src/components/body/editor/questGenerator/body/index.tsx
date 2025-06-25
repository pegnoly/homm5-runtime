import styles from "../styles.module.css";
import QuestGeneratorDataBlock from "./data";
import QuestGeneratorProgressBlock from "./progress";

function QuestGeneratorBody() {
    return (
    <div className={styles.body}>
        <div className={styles.body_layout}>
            <div className={styles.progresses_panel}>
                <QuestGeneratorProgressBlock/>
            </div>
            <div className={styles.data_panel}>
                <QuestGeneratorDataBlock/>
            </div>
        </div>
    </div>
    )
}

export default QuestGeneratorBody;