import { Col, Row } from "antd";
import { useParams } from "react-router";
import HeroAssetArtifactsConfigurator from "./artsConfigurator/main";
import HeroAssetStacksConfigurator from "./stacksConfigurator/main";
import { useEffect, useState } from "react";
import { HeroAssetSimple } from "./main";
import { invoke } from "@tauri-apps/api/core";
import useHeroGeneratorStore from "../../stores/FightGeneratorStore";

function HeroAssetFocused() {
    const { id } = useParams();
    const [asset, setAsset] = useState<HeroAssetSimple | null>(null);
    const setCurrentAssetId = useHeroGeneratorStore((state) => state.setCurrentAssetId);

    useEffect(() => {
        if (id != undefined) {
            loadAsset();
        }
    }, [id])

    const loadAsset = async () => {
        await invoke<HeroAssetSimple | null>("load_hero_asset", {id: parseInt(id!)})
            .then((value) => {
                setAsset(value);
                setCurrentAssetId(value?.id!);
            });
    }

    return <>
    {
        asset != null ?
        <div style={{width: '100%', height: '100%'}}>
            <div style={{width: '100%', height: '5%', justifyItems: 'center'}}>
                <div style={{border: 'solid', borderColor: 'red', borderWidth: 5, paddingInline: '3%'}}>
                    <h2>{`Focused hero - ${asset.name}`}</h2>
                </div>
            </div>
            <div style={{width: '100%', height: '84%', paddingTop: '1%'}}>
                <div style={{height: '100%'}}>
                    <Row>
                        <Col span={12}>
                            <div style={{width: '100%', height: '100%'}}>
                                <HeroAssetStacksConfigurator assetId={parseInt(id!)}/>
                            </div>
                        </Col>
                        <Col span={12}>
                            <div style={{width: '100%', height: '100%'}}>
                                <HeroAssetArtifactsConfigurator assetId={parseInt(id!)}/>
                            </div>
                        </Col>
                    </Row>
                </div>
            </div>
        </div>: 
        null
    }   
    </>
}

export default HeroAssetFocused;