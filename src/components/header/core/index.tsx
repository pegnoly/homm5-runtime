import {Button, ButtonGroup} from '@mantine/core';
import RuntimeRunner from './runtimeRunner';
import ScanerExecutor from './scanerExecutor';
import styles from '../styles.module.css'
import { invoke } from '@tauri-apps/api/core';
import {GetCurrentTimestamp, TimelineMessage} from "@/components/timeline/types.ts";
import {EditorTimelineStore} from "@/components/timeline/store.ts";

function HeaderCore() {
    const actions = EditorTimelineStore.useActions();
    async function RunValidationGenerator() {

        await invoke<TimelineMessage>("generate_validation_data")
            .then((value) => {
                actions.addItem(value);
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000)
            })
            .catch((error) => {
                actions.addItem({
                    timestamp: GetCurrentTimestamp(),
                    message: error.toString()
                })
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000)
            })
    }

    return (
    <div className={styles.core}>
        <ButtonGroup orientation='vertical'>
            <RuntimeRunner/>
            <ScanerExecutor/>
            <Button
                style={{fontSize: 10}}
                radius={0}
                onClick={() => RunValidationGenerator()}
            >
                <div style={{display:"flex", flexDirection: "column"}}>
                    <span>Generate sheets</span>
                    <span>validation</span>
                </div>
            </Button>
        </ButtonGroup>
    </div>
    )
}

export default HeaderCore;