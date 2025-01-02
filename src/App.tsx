import { Col, Row } from "antd";
import Editor from "./components/editor/Editor";
import RepackController from "./components/core/RepackController";
import MapSelector from "./components/core/MapSelector";
import RuntimeRunner from "./components/core/RuntimeRunner";

function App() {

    return <div style={{display: 'flex', flexDirection: 'column', gap: 10}}>
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
    </div>
}

export default App;