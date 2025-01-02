import { invoke } from "@tauri-apps/api/core";
import { Button, Typography } from "antd";
import { useEffect, useState } from "react";

enum RepackStage {
    Inactive,
    Active,
    Done
}

const repackStagesLabels = new Map<RepackStage, string>([
    [RepackStage.Inactive, "Inactive"],
    [RepackStage.Active, "In progress"],
    [RepackStage.Done, "Done"]
])

function getLabelColor(stage: RepackStage): string {
    switch (stage) {
        case RepackStage.Inactive:
            return "black"
            break;
        case RepackStage.Active:
            return "red"
            break;
        case RepackStage.Done:
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

    const [stage, setStage] = useState<RepackStage>(RepackStage.Inactive)

    async function runRepack() {
        setStage(RepackStage.Active);
        invoke("repack", {repackerLabel: label})
            .then(() => {
                setStage(RepackStage.Done)
                setTimeout(() => {
                    setStage(RepackStage.Inactive)
                }, 2500)
            })
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 3}}>
        <Button
            disabled={stage != RepackStage.Inactive}
            onClick={runRepack}
        >{`Repack ${label}`}</Button>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 13, color: getLabelColor(stage), textAlign: 'center'}}>{repackStagesLabels.get(stage)}</Typography.Text>
    </div>
}

export default RepackController;