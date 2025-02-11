import { invoke } from "@tauri-apps/api/core";
import { Button, Typography } from "antd";
import { useEffect, useState } from "react";

export enum WorkStage {
    Inactive,
    Active,
    Done
}

export const repackStagesLabels = new Map<WorkStage, string>([
    [WorkStage.Inactive, "Inactive"],
    [WorkStage.Active, "In progress"],
    [WorkStage.Done, "Done"]
])

export function getLabelColor(stage: WorkStage): string {
    switch (stage) {
        case WorkStage.Inactive:
            return "black"
            break;
        case WorkStage.Active:
            return "red"
            break;
        case WorkStage.Done:
            return "green"
            break
        default:
            return "black"
            break;
    }
}

function RepackController() {

    const [repackers, setRepackers] = useState<string[]>([]);

    useEffect(() => {
        if (repackers.length == 0) {
            invoke<string[]>("load_repackers").then((rs) => setRepackers(rs))
        }
    }, [repackers])

    return <div style={{display: 'flex', flexDirection: 'row', gap: 5, justifyContent: 'space-between'}}>{repackers.map((r, i) => (
        <Repacker key={i} label={r}/>
    ))}</div>
}

function Repacker({label} : {label: string}) {

    const [stage, setStage] = useState<WorkStage>(WorkStage.Inactive)

    async function runRepack() {
        setStage(WorkStage.Active);
        invoke("repack", {repackerLabel: label})
            .then(() => {
                setStage(WorkStage.Done)
                setTimeout(() => {
                    setStage(WorkStage.Inactive)
                }, 2500)
            })
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 3}}>
        <Button
            disabled={stage != WorkStage.Inactive}
            onClick={runRepack}
        >{`Repack ${label}`}</Button>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 13, color: getLabelColor(stage), textAlign: 'center'}}>{repackStagesLabels.get(stage)}</Typography.Text>
    </div>
}

export default RepackController;