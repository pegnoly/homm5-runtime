import { SimpleGrid } from '@mantine/core';
import styles from '../styles.module.css'
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import RepackItem from './repackItem';

type RepackerData = {
    label: string,
    update_time: string
}

function RepackersPanel() {
    const [repackers, setRepackers] = useState<RepackerData[]>([]);

    useEffect(() => {
        if (repackers.length == 0) {
            invoke<RepackerData[]>("load_repackers").then((rs) => setRepackers(rs))
        }
    }, [repackers])

    async function updateRepackTime(label: string, time: string) {
        const updatedRepackers = repackers.map((r) => {
            if (r.label == label) {
                r.update_time = time;
            }
            return r;
        });
        setRepackers(updatedRepackers);
    }

    return (
    <div className={styles.repackers}>
        <SimpleGrid cols={{xl: 4, sm: 3}}>{repackers.map((r, i) => (
            <RepackItem key={i} label={r.label} time={r.update_time} updateCallback={updateRepackTime}/>
        ))}</SimpleGrid>
    </div>
    )
}

export default RepackersPanel;