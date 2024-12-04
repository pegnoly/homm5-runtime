import { invoke } from "@tauri-apps/api/core";
import { Button, Carousel, Col, Grid, Input, Row, Typography } from "antd";
import TextArea from "antd/es/input/TextArea";
import { useEffect, useState } from "react";
import { useQuestStore } from "./stores/QuestStore";
import { useShallow } from "zustand/shallow";
import { listen } from "@tauri-apps/api/event";

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
        <Button style={{width: '50%', height: 30}} onClick={runGame}>Запустить игру</Button>
        <Button style={{width: '50%', height: 30}} onClick={executeScan}>Сканировать файлы</Button>
        <div style={{paddingTop: 30, justifyItems: 'center'}}>{repackers.map((r, i) => (
            <RepackController key={i} label={r}/>
        ))}</div>
        <QuestGenerator/>
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
        <Button
        disabled={stage != RepackStage.Inactive}
        onClick={runRepack}>{`Запаковать ${label}`}</Button>
        <Typography.Text style={{color: getLabelColor(stage), textAlign: 'center'}}>{repackStagesLabels.get(stage)}</Typography.Text>
    </div>
}

function QuestGenerator() {
    return <>
        <Row gutter={10}>
            <Col span={10}>
                <QuestDataDisplayer/>
            </Col>
            <Col span={14}>
                <QuestProgressDisplayer/>
            </Col>
        </Row>
    </>
}

function QuestDataDisplayer() {

    const [script_name, name, desc, directory, progresses, set_script_name, set_name, set_desc, set_directory] = useQuestStore(useShallow((state) => [
        state.script_name, state.name, state.desc, state.directory, state.progresses, state.set_script_name, state.set_name, state.set_desc, state.set_directory
    ]))

    function initNewQuest() {
        set_script_name("")
        set_name("")
        set_desc("")
        set_directory("")
    }

    function tryPickDirectory() {
        invoke("pick_quest_directory")
    }

    function tryGenerate() {
        invoke("generate_quest", {directory: directory, scriptName: script_name, name: name, desc: desc, progresses: progresses})
    }

    listen<string>("quest_directory_picked", (event) => {
        set_directory(event.payload)
    })

    return <>   
        <div style={{paddingBottom: 2}}>
            <Button onClick={initNewQuest}>Новый</Button> 
        </div>
        <Typography.Text>Скриптовое имя квеста</Typography.Text>
        <Input onChange={(e) => set_script_name(e.currentTarget.value)} value={script_name}/>
        <Button onClick={tryPickDirectory}>Указать путь к папке с квестом</Button>
        <div style={{paddingTop: 2}}>
            <Typography.Text>Имя квеста</Typography.Text>
            <Input onChange={(e) => set_name(e.currentTarget.value)} value={name}/>
        </div>
        <Typography.Text>Описание квеста</Typography.Text>
        <TextArea onChange={(e) => set_desc(e.currentTarget.value)} value={desc} rows={10}/>
        <Button onClick={tryGenerate}>Сгенерировать</Button>
    </>
}

function QuestProgressDisplayer() {
    const [progress, setProgress] = useState<number>(0);

    return <>
        <TextArea rows={15}/>
        <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', gap: 10, paddingTop: 15}}>
            <Button 
                disabled={progress == 0}
                onClick={() => setProgress(progress - 1)}
            >Предыдущий</Button>
            <Typography.Text>{progress}</Typography.Text>
            <Button
                onClick={() => setProgress(progress + 1)}
            >Следующий</Button>
        </div>
    </>
}

export default App;