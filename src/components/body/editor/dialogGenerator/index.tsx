import { useQuery } from '@tanstack/react-query';
import styles from '../../styles.module.css';
import DialogGeneratorHeader from './header';
import { DialogGeneratorApi } from './api';
import {DialogGeneratorStore} from './store';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Speaker } from './types';
import DialogGeneratorBody from './body';
import {useCurrentMapId} from "@/stores/common.ts";

function DialogGeneratorLayout() {
    const actions = DialogGeneratorStore.useActions();
    const speakers = DialogGeneratorStore.useSpeakers();
    const dialogId = DialogGeneratorStore.useCurrentDialogId();
    const currentMapId = useCurrentMapId();

    useEffect(() => {
        loadSpeakers().then(() => {});
    }, []);

    const loadSpeakers = async() => {
        invoke<Speaker[]>("load_speakers", { mapId: currentMapId })
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
    const actions = DialogGeneratorStore.useActions();
    const dialogId = DialogGeneratorStore.useCurrentDialogId();

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