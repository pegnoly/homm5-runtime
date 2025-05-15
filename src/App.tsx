import { Col, Row } from "antd";
import Editor from "./components/editor/Editor";
import RepackController from "./components/core/RepackController";
import MapSelector from "./components/core/MapSelector";
import RuntimeRunner from "./components/core/RuntimeRunner";
import useGameDataStore, { ArtifactModel, CreatureModel } from "./stores/GameDataStore";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import { useShallow } from "zustand/shallow";

function App() {
    const [setArtifacts, setCreatures] = useGameDataStore(useShallow((state) => [state.load_artifacts, state.load_creatures]));

    useEffect(() => {
        loadArtifactModels();
        loadCreatureModels();
    }, [])

    const loadArtifactModels = async () => {
        await invoke<ArtifactModel[]>("load_artifact_models")
            .then((values) => setArtifacts(values));
    }

    const loadCreatureModels = async () => {
        await invoke<CreatureModel[]>("load_creature_models")
            .then((values) => setCreatures(values));
    }

    return <div style={{display: 'flex', flexDirection: 'column', height: '100%', gap: 10}}>
        <Row>
            <Col span={12}>
                <RuntimeRunner/>
            </Col>
            <Col span={8} offset={4}>
                <MapSelector/>
            </Col>
        </Row>
        <RepackController/>
        <Editor/>
        {/* <Button onClick={() => Test()}>Test</Button> */}
    </div>
}

export default App;