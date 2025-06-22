import { useQuery } from '@tanstack/react-query';
import styles from '../../styles.module.css';
import DialogGeneratorHeader from './header';
import { DialogGeneratorApi } from './api';
import { useCurrentDialogId, useDialogActions, useSpeakers } from './store';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Speaker } from './types';
import DialogGeneratorBody from './body';

function DialogGeneratorLayout() {
    const actions = useDialogActions();
    const speakers = useSpeakers();
    const dialogId = useCurrentDialogId();

    useEffect(() => {
        loadSpeakers();
    }, []);

    const loadSpeakers = async() => {
        invoke<Speaker[]>("load_speakers")
            .then((data) => {
                actions.loadSpeakers(data);
            });
    }

    return (
    <div className={styles.editor_layout}>
        {
            speakers == undefined ?
            null :
            <>
                <DialogGeneratorHeader/>
                <DialogGeneratorBody/>
                {
                    dialogId == undefined ?
                    null :
                    <DialogLoader/>
                }
            </>
        }
    </div>
    )
}

function useDialog(id: number) {
    return useQuery({
        queryKey: ["dialog", id],
        queryFn: async() => {
            return DialogGeneratorApi.loadDialog(id);
        }
    })
}

function DialogLoader() {
    const actions = useDialogActions();
    const dialogId = useCurrentDialogId();

    const { data } = useDialog(dialogId!);
    if (data == undefined) {
        return null;
    }
    
    actions.loadCurrentDialog(data);
    actions.setCurrentVariantStep(0);
    actions.setCurrentVariantLabel("main");

    return null;
}

export default DialogGeneratorLayout;