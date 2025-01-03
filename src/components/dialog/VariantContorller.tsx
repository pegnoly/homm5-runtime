import { Button, Select, Typography } from "antd"
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore"
import { useShallow } from "zustand/shallow"
import { invoke } from "@tauri-apps/api/core"
import { useSpeakersStore } from "../../stores/SpeakersStore"
import { useEffect } from "react"

function DialogVariantController() {

    const speakers = useSpeakersStore((state) => state.speakers)

    const [currentDialogId, setSpeakers, setLabels] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_id,
        state.set_current_dialog_speakers,
        state.set_current_dialog_labels
    ]))

    useEffect(() => {
        if (currentDialogId != null) {
            loadSpeakers()
            loadLabels()
        }
    }, [currentDialogId])
    
    const loadSpeakers = async () => {
        await invoke<string[]>("load_dialog_speakers", {dialogId: currentDialogId})
            .then((ids) => setSpeakers(speakers.filter((sp) => ids.includes(sp.id))))
    }

    const loadLabels = async () => {
        await invoke<string[]>("load_dialog_labels", {dialogId: currentDialogId})
            .then((labels) => setLabels(labels))
    }

    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
        <StepSwitcher/>
        <LabelSelector/>
    </div>
}

function StepSwitcher() {

    const [currentDialogId, setCurrentVariantId, currentStep, setCurrentStep, currentLabel] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_id,
        state.set_current_variant_id,
        state.current_step,
        state.set_current_step,
        state.current_label
    ]))

    async function updateStep(change: number) {
        setCurrentStep(currentStep + change)
        await invoke<string>("load_dialog_variant", {dialogId: currentDialogId, dialogStep: currentStep + change, label: currentLabel})
            .then((id) => setCurrentVariantId(id))
    }

    return <div style={{width: '50%', display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
        <Button 
            style={{width: '40%'}}
            disabled={currentStep == 0}
            onClick={() => updateStep(-1)}
        >Previous</Button>
        <Typography.Text>{currentStep}</Typography.Text>
        <Button 
            style={{width: '40%'}}
            onClick={() => updateStep(1)}
        >Next</Button>
    </div>
}

function LabelSelector() {

    const [labels, currentDialogId, currentStep, setCurrentVariantId, currentLabel, setCurrentLabel] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_labels,
        state.current_dialog_id, 
        state.current_step, 
        state.set_current_variant_id,
        state.current_label,
        state.set_current_label
    ]))

    async function selectLabel(label: string) {
        setCurrentLabel(label)
        await invoke<string>("load_dialog_variant", {dialogId: currentDialogId, dialogStep: currentStep, label: label})
            .then((id) => setCurrentVariantId(id))
    }

    return <>
        <Select 
            style={{width: '30%'}}
            value={currentLabel}
            onChange={selectLabel}
        >{labels.map((l, i) => (
            <Select.Option key={i} value={l}>{l}</Select.Option>
        ))}</Select>
    </>
}

export default DialogVariantController;