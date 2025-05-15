import { invoke } from "@tauri-apps/api/core";
import { List, Typography } from "antd";
import { useEffect, useState } from "react";
import { Link, Route, Routes } from "react-router";
import HeroAssetFocused from "./focused";
import HeroAssetCreator from "./creator";

export type HeroAssetSimple = {
    id: number,
    name: string
}

function HeroGeneratorMain() {
    const [heroes, setHeroes] = useState<HeroAssetSimple[]>([]);

    useEffect(() => {
        loadHeroes();
    }, []);

    const loadHeroes = async () => {
        invoke<HeroAssetSimple[]>("load_all_hero_assets")
            .then((values) => setHeroes(values));
    }

    async function onAssetCreated(asset: HeroAssetSimple) {
        setHeroes([...heroes, asset]);
    }

    return <div style={{overflow: 'hidden', display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
        <Routes>
            <Route path="/" element={<HeroAssetsList assets={heroes}/>}/>
            <Route path="/asset/:id" element={<HeroAssetFocused/>}/>
        </Routes>
        <HeroAssetCreator assetCreatedCallback={onAssetCreated}/>
    </div>
}

function HeroAssetsList(params: {assets: HeroAssetSimple[]}) {

    return <div style={{display: 'flex', flexDirection: 'column', height: '100%', alignItems: 'center'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded', letterSpacing: 2}}>Assets list</Typography.Text>
            <List>{params.assets.map((asset, index) => (
            <Link key={index} to={`asset/${asset.id}`}>
                <List.Item style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 18}}>{asset.name}</List.Item>
            </Link>
        ))}</List>
    </div>
}

export default HeroGeneratorMain;