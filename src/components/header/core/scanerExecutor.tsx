import { useState } from "react";
import { WorkStage } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@mantine/core";

function ScanerExecutor() {
    const [scanStage, setScanStage] = useState<WorkStage>(WorkStage.Inactive)

    async function executeScan() {
        setScanStage(WorkStage.Active)
        await invoke("execute_scan")
            .then(() => setScanStage(WorkStage.Inactive));
    }

    return (
    <>
        <Button
            radius={0}
            bg="grape"
            disabled={scanStage != WorkStage.Inactive} 
            onClick={executeScan}
        >Scan files</Button>
    </>
    )
}

export default ScanerExecutor;