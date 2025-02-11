import { invoke } from "@tauri-apps/api/core";
import { Button } from "antd";
import { useState } from "react";
import { WorkStage } from "./RepackController";

function RuntimeRunner() {
    
    const [scanStage, setScanStage] = useState<WorkStage>(WorkStage.Inactive)

    async function executeScan() {
        setScanStage(WorkStage.Active)
        await invoke("execute_scan").then(() => setScanStage(WorkStage.Inactive))
    }

    async function runGame() {
        await invoke("run_game")
    }
    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-between', gap: 10}}>
        <Button style={{width: '50%', height: 30}} onClick={runGame}>Run game</Button>
        <Button
            disabled={scanStage != WorkStage.Inactive} 
            style={{width: '50%', height: 30}} 
            onClick={executeScan}
        >Scan files</Button>
    </div>
}

export default RuntimeRunner;