import {Button, ButtonGroup} from '@mantine/core';
import RuntimeRunner from './runtimeRunner';
import ScanerExecutor from './scanerExecutor';
import styles from '../styles.module.css'
import { invoke } from '@tauri-apps/api/core';

function HeaderCore() {
    return (
    <div className={styles.core}>
        <ButtonGroup orientation='vertical'>
            <RuntimeRunner/>
            <ScanerExecutor/>
            <Button
                style={{fontSize: 10}}
                radius={0}
                onClick={() => invoke("generate_validation_data")}
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