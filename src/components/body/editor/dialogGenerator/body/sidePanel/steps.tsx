import { Button, ButtonGroup, Text } from "@mantine/core";
import { useCurrentDialogId, useCurrentDialogVariantStep, useDialogActions } from "../../store";

function DialogStepSwitcher() {
    const dialogId = useCurrentDialogId();
    const currentStep = useCurrentDialogVariantStep();
    const actions = useDialogActions();

    async function moveToNextStep() {
        actions.setCurrentVariantLabel("main");
        actions.setCurrentVariantStep(currentStep! + 1);
    }

    async function moveToPreviousStep() {
        actions.setCurrentVariantLabel("main");
        actions.setCurrentVariantStep(currentStep! - 1);
    }

    return (
    <>
        <Text style={{fontWeight: "bold"}}>{`Current step: ${currentStep}`}</Text>
        <ButtonGroup>
            <Button onClick={moveToPreviousStep} radius={0} disabled={currentStep == 0 || dialogId == undefined}>Previous step</Button>
            <Button onClick={moveToNextStep} radius={0} disabled={dialogId == undefined}>Next step</Button>
        </ButtonGroup>
    </>
    )
}

export default DialogStepSwitcher;