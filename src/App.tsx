import { invoke } from "@tauri-apps/api/core";
import { Button, Col, Row, Select, Typography } from "antd";
import { useEffect, useState } from "react";
import { MapData } from "./types";
import Editor from "./components/editor/Editor";

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

    return <div style={{display: 'flex', flexDirection: 'column', gap: 10}}>
        <Button style={{width: '50%', height: 30}} onClick={runGame}>Запустить игру</Button>
        <Button style={{width: '50%', height: 30}} onClick={executeScan}>Сканировать файлы</Button>
        <Row>
            <Col span={12}>
                <div style={{paddingTop: 15}}>{repackers.map((r, i) => (
                    <RepackController key={i} label={r}/>
                ))}</div>
            </Col>
            <Col span={12}>
                <div style={{paddingTop: 15}}>
                    <MapSelector/>
                </div>
            </Col>
        </Row>
        <Editor/>
    </div>
}

function MapSelector() {

    const [maps, setMaps] = useState<MapData[]>([])
    const [currentMapId, setCurrentMapId] = useState<number | null>(null);

    useEffect(() => {
        if (maps.length == 0) {
            invoke("load_maps")
                .then((maps_value) => {
                    setMaps(maps_value as MapData[]);
                    invoke("load_current_map")
                        .then((id_value) => {
                            if (id_value != null) {
                                setCurrentMapId(id_value as number);
                            }
                        })
                })
        }
    }, [maps])

    async function selectMap(map_id: number) {
        setCurrentMapId(map_id)
        await invoke("select_map", {id: map_id});
    }

    return <>
        <Typography.Text>Текущая карта</Typography.Text>
        <Select 
            value={currentMapId}
            onChange={selectMap}
        >{maps.map((m, i) => (
            <Select.Option key={i} value={m.id}>{m.name}</Select.Option>
        ))}</Select>
    </>
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

export default App;