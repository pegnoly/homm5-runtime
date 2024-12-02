import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

enum AppState {
    NotReady,
    Ready
}

function App() {

    const [repackers, setRepackers] = useState<string[]>([]);
    const [state, setState] = useState<AppState>(AppState.NotReady);

    useEffect(() => {
        if (state == AppState.NotReady) {
            setState(AppState.Ready)
            invoke("load_repackers")
                .then((rs) => setRepackers(rs as string[]))
        }
    }, [state])

    async function executeScan() {
        await invoke("execute_scan")
    }

    async function runGame() {
        await invoke("run_game")
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 10, alignItems: 'center'}}>
        <button style={{width: '50%', height: 30}} onClick={runGame}>Запустить игру</button>
        <button style={{width: '50%', height: 30}} onClick={executeScan}>Сканировать файлы</button>
        <div style={{paddingTop: 30, justifyItems: 'center'}}>{repackers.map((r, i) => (
            <RepackController key={i} label={r}/>
        ))}</div>
    </div>
}


enum RepackStage {
    Inactive,
    Active,
    Done
}

const repackStagesLabels = new Map<RepackStage, string>([
    [RepackStage.Inactive, "Неактивно"],
    [RepackStage.Active, "В процессе"],
    [RepackStage.Done, "Готово"]
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

function RepackController({label}: {label: string}) {
    
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

    return <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
        <button
        disabled={stage != RepackStage.Inactive}
        onClick={runRepack}>{`Запаковать ${label}`}</button>
        <label style={{color: getLabelColor(stage)}}>{repackStagesLabels.get(stage)}</label>
    </div>
}

export default App;