import { Col, Row } from "antd";
import { useParams } from "react-router";
import HeroAssetStacksConfigurator from "./stacksConfigurator";
import HeroAssetArtifactsConfigurator from "./artsConfigurator/main";

function HeroAssetFocused() {
    const { id } = useParams();

    return <div style={{width: '100%', height: '100%'}}>
        <div style={{width: '100%', height: '20%'}}>
            <h1>{`Focused ${id}`}</h1>
        </div>
        <div style={{width: '100%', height: '79%'}}>
            <Row>
                <Col span={12}>
                    <HeroAssetStacksConfigurator assetId={parseInt(id!)}/>
                </Col>
                <Col span={12}>
                    <HeroAssetArtifactsConfigurator assetId={parseInt(id!)}/>
                </Col>
            </Row>
        </div>
    </div>
}

export default HeroAssetFocused;