import { useQuery } from "@tanstack/react-query";
import {
    DialogGeneratorStore
} from "../store";
import styles from "../styles.module.css";
import DialogGeneratorTextArea from "./textArea";
import { DialogGeneratorApi } from "../api";
import DialogStepSwitcher from "./sidePanel/steps";
import DialogStepSaver from "./sidePanel/save";
import DialogStepCurrent from "@/components/body/editor/dialogGenerator/body/sidePanel/current.tsx";
import DialogUpdater from "@/components/body/editor/dialogGenerator/body/sidePanel/update.tsx";

export type GetVariantPayload = {
    dialogId: number,
    step: number,
    label: string
}

export function useVariant(payload: GetVariantPayload) {
    return useQuery({
        queryKey: ["dialog_variant", payload.dialogId, payload.step, payload.label],
        queryFn: async() => {
            return DialogGeneratorApi.tryLoadVariant(payload);
        }
    })
}

function DialogGeneratorBody() {
    const dialogId = DialogGeneratorStore.useCurrentDialogId();

    return (
        <>
            {dialogId == undefined ? <div style={{position: 'absolute', top: '50%', left: '50%'}}>
                <h2>Нет активного диалога</h2>
            </div> :
            <div className={styles.body}>
                <div className={styles.body_layout}>
                    <div className={styles.textarea}>
                        <DialogGeneratorTextArea/>
                    </div>
                    <div className={styles.side_panel}>
                        <div className={styles.side_panel_layout}>
                            <div style={{height: "25%", display: 'flex', flexDirection: 'column', gap: "10%"}}>
                                <DialogStepSaver/>
                                <DialogStepSwitcher/>
                            </div>
                            <div style={{height: "30%"}}>
                                <DialogStepCurrent/>
                            </div>
                            <div style={{height: "25%"}}>
                                <DialogUpdater/>
                            </div>
                        </div>
                    </div>
                </div>
                <VariantLoader/>
            </div>
            }
        </>
    )
}

function VariantLoader() {
    const dialogId = DialogGeneratorStore.useCurrentDialogId();
    const step = DialogGeneratorStore.useCurrentVariantStep();
    const label = DialogGeneratorStore.useCurrentVariantLabel();
    const actions = DialogGeneratorStore.useActions();

    const { data } = useVariant({dialogId: dialogId!, label: label!, step: step!});
    if (data == undefined) 
        return null;
    actions.loadVariant(data);

    return null;
}

export default DialogGeneratorBody;