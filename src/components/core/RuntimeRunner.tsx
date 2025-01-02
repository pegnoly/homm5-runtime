import { invoke } from "@tauri-apps/api/core";
import { Button } from "antd";

function RuntimeRunner() {
    
    async function executeScan() {
        await invoke("execute_scan")
    }

    async function runGame() {
        await invoke("run_game")
    }
    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-between', gap: 10}}>
        <Button style={{width: '50%', height: 30}} onClick={runGame}>Run game</Button>
        <Button style={{width: '50%', height: 30}} onClick={executeScan}>Scan files</Button>
    </div>
}

export default RuntimeRunner;