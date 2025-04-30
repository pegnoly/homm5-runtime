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

type RepackerData = {
    label: string,
    update_time: string
}

function RepackController() {

    const [repackers, setRepackers] = useState<RepackerData[]>([]);

    useEffect(() => {
        if (repackers.length == 0) {
            invoke<RepackerData[]>("load_repackers").then((rs) => setRepackers(rs))
        }
    }, [repackers])

    async function updateRepackTime(label: string, time: string) {
        const updatedRepackers = repackers.map((r) => {
            if (r.label == label) {
                r.update_time = time;
            }
            return r;
        });
        setRepackers(updatedRepackers);
    }

    return <div style={{display: 'flex', flexDirection: 'row', gap: 5, justifyContent: 'space-between'}}>{repackers.map((r, i) => (
        <Repacker key={i} label={r.label} time={r.update_time} updateCallback={updateRepackTime}/>
    ))}</div>
}

function Repacker(params : {label: string, time: string, updateCallback: (label: string, time: string) => void}) {

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
            disabled={stage != WorkStage.Inactive}
            onClick={runRepack}
        >{`Repack ${params.label}`}</Button>
        <Typography.Text 
            style={{fontFamily: 'fantasy', fontSize: 13, color: getLabelColor(stage), textAlign: 'center'}}
        >{stage == WorkStage.Inactive ? params.time : repackStagesLabels.get(stage)}</Typography.Text>
    </div>
}

export default RepackController;