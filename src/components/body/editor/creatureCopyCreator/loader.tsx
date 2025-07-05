import { Button } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { SessionConfig } from "./types";
import CreatureCopyCreator from "./store";


function CreatureGenerationSessionLoader() {
    const actions = CreatureCopyCreator.useActions();

    listen<string | null>("session_file_picked", e => {
        invoke<SessionConfig>("load_session", {sessionFile: e.payload!})
            .then((value) => actions.loadSession(value));
    })

    async function pickFile() {
        await invoke("pick_session_file");
    }

    return (
    <>
        <Button 
            onClick={pickFile}
            radius={0}
            bg="orange"
        >Load session</Button>
    </>
    )
}

export default CreatureGenerationSessionLoader;