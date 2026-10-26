import { Tabs } from "@mantine/core";
import styles from "../styles.module.css";
import QuestGeneratorDataBlock from "./data";
import QuestGeneratorProgressBlock from "./progress";
import QuestGeneratorUpdateSourcesBlock from "./updateSources";
import {QuestGeneratorStore} from "@/components/body/editor/questGenerator/store.ts";

enum QuestGeneratorBodyTab {
    Data = "data",
    UpdateSources = "update_sources"
}

function QuestGeneratorBody() {
    const currentQuestId = QuestGeneratorStore.useQuestId();
    return (
        currentQuestId == undefined ? <h1 style={{position: 'absolute', top: '50%', left: '50%'}}>НЕТ АКТИВНОГО КВЕСТА</h1> :
    <div className={styles.body}>
        <div className={styles.body_layout}>
            <div className={styles.progresses_panel}>
                <QuestGeneratorProgressBlock/>
            </div>
            <div className={styles.data_panel}>
                <Tabs
                    defaultValue={QuestGeneratorBodyTab.Data}
                    style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column'}}
                >
                    <Tabs.List style={{flexShrink: 0}}>
                        <Tabs.Tab value={QuestGeneratorBodyTab.Data}>
                            Данные квеста
                        </Tabs.Tab>
                        <Tabs.Tab value={QuestGeneratorBodyTab.UpdateSources}>
                            Источники обновления
                        </Tabs.Tab>
                    </Tabs.List>
                    <Tabs.Panel
                        value={QuestGeneratorBodyTab.Data}
                        style={{flex: 1, minHeight: 0, overflow: 'hidden'}}
                    >
                        <QuestGeneratorDataBlock/>
                    </Tabs.Panel>
                    <Tabs.Panel
                        value={QuestGeneratorBodyTab.UpdateSources}
                        style={{flex: 1, minHeight: 0, overflow: 'hidden'}}
                    >
                        <QuestGeneratorUpdateSourcesBlock/>
                    </Tabs.Panel>
                </Tabs>
            </div>
        </div>
    </div>
    )
}

export default QuestGeneratorBody;
