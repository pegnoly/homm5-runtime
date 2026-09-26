import { Button } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";
import {EditorTimelineStore} from "@/components/timeline/store.ts";
import {GetCurrentTimestamp} from "@/components/timeline/types.ts";

function RuntimeRunner () {
    const actions = EditorTimelineStore.useActions();
    async function runGame() {
        await invoke("run_game")
            .catch((e) => {
                actions.addItem({message: e.toString(), timestamp: GetCurrentTimestamp()});
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 5000)
            })
    }

    return (
    <>
        <Button 
            radius={0}
            bg="cyan"
            onClick={runGame}
        >Run game</Button>
    </>
    )
}

export default RuntimeRunner;