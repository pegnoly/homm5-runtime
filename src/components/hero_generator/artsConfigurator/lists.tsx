import { Col, Row } from "antd";
import OptionalArtifactsList from "./optional";
import RequiredArtifactsList from "./required";
import { AssetGenerationType, HeroAssetArtifactsModel, OptionalArtifacts, RequiredArtifacts } from "./types";

function HeroAssetArtifactsLists(params: {
    model: HeroAssetArtifactsModel, 
    updateCallback: (value: HeroAssetArtifactsModel) => void
}) {

    async function updateRequiredArtifacts(newArtifacts: RequiredArtifacts) {
        params.updateCallback({...params.model, required: newArtifacts});
    }

    async function updateOptionalArtifacts(newArtifacts: OptionalArtifacts) {
        params.updateCallback({...params.model, optional: newArtifacts});
    }

    return <div>
        <Row>
            <Col span={11}>
                <RequiredArtifactsList modelId={params.model.id} currentArtifacts={params.model.required} updateCallback={updateRequiredArtifacts}/>
            </Col>
            {
                params.model.generation_type == AssetGenerationType.Dynamic ?
                <Col offset={1} span={11}>
                    <OptionalArtifactsList modelId={params.model.id} currentArtifacts={params.model.optional} updateCallback={updateOptionalArtifacts}/>
                </Col> :
                null
            }
        </Row>
    </div>
}

export default HeroAssetArtifactsLists;