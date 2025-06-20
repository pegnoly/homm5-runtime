import { ButtonGroup } from '@mantine/core';
import RuntimeRunner from './runtimeRunner';
import ScanerExecutor from './scanerExecutor';
import styles from '../styles.module.css'

function HeaderCore() {
    return (
    <div className={styles.core}>
        <ButtonGroup orientation='vertical'>
            <RuntimeRunner/>
            <ScanerExecutor/>
        </ButtonGroup>
    </div>
    )
}

export default HeaderCore;