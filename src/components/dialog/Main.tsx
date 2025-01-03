import { useShallow } from "zustand/shallow";
import { Speaker, useSpeakersStore } from "../../stores/SpeakersStore";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import DialogInitializator from "./Initializator";
import DialogVariantController from "./VariantContorller";
import VariantRenderer from "./VariantRenderer";

function DialogGeneratorMain() {

    const [speakers, setSpeakers] = useSpeakersStore(useShallow((state) => [state.speakers, state.set_speakers]))

    useEffect(() => {
        if (speakers.length == 0) {
            loadSpeakers()
        }
    }, [speakers])

    const loadSpeakers = async () => {
        await invoke<Speaker[]>("load_speakers").then((sps) => setSpeakers(sps))
    }

    return <>
        <DialogInitializator/>
        <DialogVariantController/>
        <VariantRenderer/>
    </>
}

export default DialogGeneratorMain;