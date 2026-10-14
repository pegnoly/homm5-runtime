import { Button, ButtonGroup, Text } from "@mantine/core";
import {DialogGeneratorStore} from "@/components/body/editor/dialogGenerator/store.ts";

function DialogStepSwitcher() {
    const dialogId = DialogGeneratorStore.useCurrentDialogId();
    const currentStep = DialogGeneratorStore.useCurrentVariantStep();
    const isSaved = DialogGeneratorStore.useCurrentVariantSaved();
    const actions = DialogGeneratorStore.useActions();

    async function moveToNextStep() {
        actions.setCurrentVariantLabel("main");
        actions.setCurrentVariantSpeakers([]);
        actions.setCurrentVariantStep(currentStep! + 1);
    }

    async function moveToPreviousStep() {
        actions.setCurrentVariantLabel("main");
        actions.setCurrentVariantStep(currentStep! - 1);
    }

    return (
    <>
        <Text style={{fontWeight: "bold", textAlign: 'center'}}>{`Текущая реплика: ${currentStep}`}</Text>
        <ButtonGroup>
            <Button 
                onClick={moveToPreviousStep} 
                radius={0} 
                disabled={currentStep == 0 || dialogId == undefined || !isSaved}
            >Предыдущая</Button>
            <Button 
                onClick={moveToNextStep} 
                radius={0} 
                disabled={dialogId == undefined || !isSaved}
            >Следующая</Button>
        </ButtonGroup>
    </>
    )
}

export default DialogStepSwitcher;