import { Button, ButtonGroup } from '@mantine/core';
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
            <Button onClick={() => invoke("generate_images")}>Test shit</Button>
        </ButtonGroup>
    </div>
    )
}

export default HeaderCore;