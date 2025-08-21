import { useQuery } from "@tanstack/react-query";
import { useCurrentDialogId, useCurrentDialogVariantLabel, useCurrentDialogVariantStep, useDialogActions } from "../store";
import styles from "../styles.module.css";
import DialogGeneratorTextArea from "./textArea";
import { DialogGeneratorApi } from "../api";
import CurrentDialogLabels from "./sidePanel/labels";
import DialogStepSwitcher from "./sidePanel/steps";
import DialogStepSpeakerSelector from "./sidePanel/speaker";
import DialogStepSaver from "./sidePanel/save";

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
    const dialogId = useCurrentDialogId();

    return (
    <div className={styles.body}>
        <div className={styles.body_layout}>
            <div className={styles.textarea}>
                <DialogGeneratorTextArea/>
            </div>
            <div className={styles.side_panel}>
                <div className={styles.side_panel_layout}>
                    <div style={{height: "25%"}}>
                        <DialogStepSpeakerSelector/>
                    </div>
                    <div style={{height: "30%"}}>
                        <CurrentDialogLabels/>
                    </div>
                    <div style={{height: "25%"}}>
                        <DialogStepSwitcher/>
                    </div>
                    <DialogStepSaver/>
                </div>
            </div>
        </div>
        {
            dialogId == undefined ?
            null :
            <VariantLoader/>
        }
    </div> 
    )
}

function VariantLoader() {
    const dialogId = useCurrentDialogId();
    const step = useCurrentDialogVariantStep();
    const label = useCurrentDialogVariantLabel();
    const actions = useDialogActions();

    const { data } = useVariant({dialogId: dialogId!, label: label!, step: step!});
    if (data == undefined) 
        return null;
    actions.loadVariant(data);

    return null;
}

export default DialogGeneratorBody;