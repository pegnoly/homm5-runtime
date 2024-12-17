import { Col, Row } from "antd";
import QuestData from "./data";
import QuestProgressMain from "./progress/main";
import QuestGenerator from "./generate";

function QuestMain() {
    return <>
        <Row gutter={10}>
            <Col span={10}>
                <QuestData/>
            </Col>
            <Col span={14}>
                <QuestProgressMain/>
            </Col>
        </Row>
        <QuestGenerator/>
    </>
}

export default QuestMain;