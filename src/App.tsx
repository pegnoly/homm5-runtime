import useGameDataStore, { AbilityModel, ArtifactModel, CreatureModel, SpellModel } from "./stores/GameDataStore";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import { useShallow } from "zustand/shallow";
import Header from "./components/header";
import Body from "./components/body";

function App() {
    const [setArtifacts, setCreatures, setAbilities, setSpells] = useGameDataStore(useShallow((state) => [
        state.load_artifacts, state.load_creatures, state.load_abilities, state.load_spells
    ]));

    useEffect(() => {
        loadArtifactModels();
        loadCreatureModels();
        loadAbilityModels();
        loadSpellModels();
    }, [])

    const loadArtifactModels = async () => {
        await invoke<ArtifactModel[]>("load_artifact_models")
            .then((values) => setArtifacts(values));
    }

    const loadCreatureModels = async () => {
        await invoke<CreatureModel[]>("load_creature_models")
            .then((values) => setCreatures(values));
    }

    const loadAbilityModels = async () => {
        await invoke<AbilityModel[]>("load_abilities_models")
            .then((values) => setAbilities(values));
    }

    const loadSpellModels = async () => {
        await invoke<SpellModel[]>("load_spells_models")
            .then((values) => setSpells(values));
    }

    return (
        <>
            <Header/>
            <Body/>
        </>
    )
}

export default App;