import { ButtonGroup } from "@mantine/core";
import styles from "../styles.module.css";
import DialogCreator from "./creator";
import SpeakerCreator from "./speakerCreator";
import DialogsSelector from "./selector";
import { Dialog, Speaker } from "../types";
import { useDialogActions, useDialogs, useSpeakers } from "../store";

function DialogGeneratorHeader() {
    const speakers = useSpeakers();
    const dialogs = useDialogs();
    const actions = useDialogActions();

    async function dialogSelected(value: number) {
        actions.setCurrentDialogId(value);
    }

    async function dialogCreated(value: Dialog) {
        actions.loadDialogs([...dialogs!, value]);
        actions.loadCurrentDialog(value);
        actions.setCurrentVariantStep(0);
        actions.setCurrentVariantLabel("main");
    }

    async function speakerCreated(value: Speaker) {
        actions.loadSpeakers([...speakers!, value])
    }

    return (
    <div className={styles.header}>
        <div className={styles.manage_panel}>
            <ButtonGroup>
                <DialogCreator createdCallback={dialogCreated}/>
                <SpeakerCreator createdCallback={speakerCreated}/>
            </ButtonGroup>
            <DialogsSelector selectedCallback={dialogSelected}/>
        </div>
    </div> 
    )
}

export default DialogGeneratorHeader;