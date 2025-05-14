import { Col, Row } from "antd";
import { useParams } from "react-router";
import HeroAssetArtifactsConfigurator from "./artsConfigurator/main";
import HeroAssetStacksConfigurator from "./stacksConfigurator/main";
import { useEffect, useState } from "react";
import { HeroAssetSimple } from "./main";
import { invoke } from "@tauri-apps/api/core";
import { useHeroGeneratorStore } from "../../stores/HeroGeneratorStore";

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
            <div style={{width: '100%', height: '20%'}}>
                <h1>{`Focused hero - ${asset.name}`}</h1>
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
        </div>: 
        null
    }   
    </>
}

export default HeroAssetFocused;