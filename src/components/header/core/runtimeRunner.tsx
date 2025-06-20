import { Button } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";

function RuntimeRunner () {
    async function runGame() {
        await invoke("run_game")
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