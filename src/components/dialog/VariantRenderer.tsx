import { Button, Select } from "antd"
import { useCurrentDialogStore } from "../../stores/CurrentDialogStore"
import { useShallow } from "zustand/shallow"
import { invoke } from "@tauri-apps/api/core"
import { useEffect } from "react"
import TextArea from "antd/es/input/TextArea"


function VariantRenderer() {

    const [currentDialogId, setCurrentVariantId] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_id, state.set_current_variant_id
    ]))

    // this component must react on change of current dialog and request of variant with step 0 and label "main"
    useEffect(() => {
        if (currentDialogId != null) {
            loadDefaultVariant()
        }
    }, [currentDialogId])
    
    const loadDefaultVariant = async () => {
        await invoke<string>("load_dialog_variant", {dialogId: currentDialogId, dialogStep: 0, label: "main"})
            .then((id) => setCurrentVariantId(id))
    }

    return <div style={{padding: '3%'}}>
        <VariantText/>
        <div style={{paddingTop: '2%', display: 'flex', justifyContent: 'space-between'}}>
            <VariantSettings/>
            <VariantSaver/>
        </div>
    </div>
}

function VariantText() {

    const [currentVariantId, currentVariantText, setCurrentVariantText] = useCurrentDialogStore(useShallow((state) => [
        state.current_variant_id,
        state.current_variant_text,
        state.set_current_variant_text
    ]))

    useEffect(() => {
        if (currentVariantId != null) {
            loadText()
        }
    }, [currentVariantId])

    const loadText = async () => {
        await invoke<string>("load_variant_text", {variantId: currentVariantId})
            .then((text) => setCurrentVariantText(text))
    }

    return <>
        <TextArea
            value={currentVariantText}
            onChange={(e) => setCurrentVariantText(e.currentTarget.value)}
            rows={18}
        />
    </>
}

function VariantSettings() {
    
    const [speakers, currentVariantId, currentVariantSpeaker, setCurrentVariantSpeaker] = useCurrentDialogStore(useShallow((state) => [
        state.current_dialog_speakers,
        state.current_variant_id,
        state.current_variant_speaker,
        state.set_current_variant_speaker
    ]))

    useEffect(() => {
        if (currentVariantId != null) {
            loadSpeaker()
        }
    }, [currentVariantId])

    const loadSpeaker = async () => {
        await invoke<string | null>("load_variant_speaker", {variantId: currentVariantId})
            .then((id) => setCurrentVariantSpeaker(id))
    }

    async function selectSpeaker(speaker_id: string) {
        setCurrentVariantSpeaker(speaker_id)
    }

    return <>
        <Select
            style={{width: '30%'}}
            value={currentVariantSpeaker}
            onChange={selectSpeaker}
        >{speakers.map((s, i) => (
            <Select.Option key={i} value={s.id}>{s.name}</Select.Option>
        ))}</Select>
    </>
}

function VariantSaver() {

    const [currentVariantId, currentVariantSpeaker, currentVariantText] = useCurrentDialogStore(useShallow((state) => [
        state.current_variant_id,
        state.current_variant_speaker,
        state.current_variant_text
    ]))

    async function saveVariant() {
        await invoke("save_dialog_variant", {variantId: currentVariantId, speaker: currentVariantSpeaker, text: currentVariantText})
    }

    return <>
        <Button
            disabled={!currentVariantSpeaker}
            onClick={saveVariant}
        >Save this variant</Button>
    </>
}

export default VariantRenderer;