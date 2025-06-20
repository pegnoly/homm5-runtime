import { useState } from "react";
import { repackStagesLabels, WorkStage } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Button, Text } from "@mantine/core";
import { getLabelColor } from "../utils";

function RepackItem(params: {
    label: string,
    time: string,
    updateCallback: (label: string, time: string) => void
}) {
    const [stage, setStage] = useState<WorkStage>(WorkStage.Inactive)

    async function runRepack() {
        setStage(WorkStage.Active);
        invoke<string>("repack", {repackerLabel: params.label})
            .then((value) => {
                setStage(WorkStage.Done)
                setTimeout(() => {
                    setStage(WorkStage.Inactive);
                    params.updateCallback(params.label, value);
                }, 2500);
            })
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 3}}>
        <Button
            radius={0}
            size="xs"
            disabled={stage != WorkStage.Inactive}
            onClick={runRepack}
        >{`Repack ${params.label}`}</Button>
        <Text 
            style={{fontFamily: 'fantasy', fontSize: 13, color: getLabelColor(stage), textAlign: 'center'}}
        >{stage == WorkStage.Inactive ? params.time : repackStagesLabels.get(stage)}</Text>
    </div>
}

export default RepackItem;