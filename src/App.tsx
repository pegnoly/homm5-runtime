import { Col, Row } from "antd";
import Editor from "./components/editor/Editor";
import RepackController from "./components/core/RepackController";
import MapSelector from "./components/core/MapSelector";
import RuntimeRunner from "./components/core/RuntimeRunner";
import useGameDataStore, { ArtifactModel } from "./stores/GameDataStore";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

function App() {
    const setArtifacts = useGameDataStore((state) => state.load_artifacts);

    useEffect(() => {
        loadArtifactModels();
    }, [])

    const loadArtifactModels = async () => {
        await invoke<ArtifactModel[]>("load_artifact_models")
            .then((values) => setArtifacts(values));
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