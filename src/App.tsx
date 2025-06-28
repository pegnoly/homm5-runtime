import useGameDataStore, { ArtifactModel, CreatureModel } from "./stores/GameDataStore";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import { useShallow } from "zustand/shallow";
import Header from "./components/header";
import Body from "./components/body";

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

    return (
        <>
            <Header/>
            <Body/>
        </>
    )
}

export default App;