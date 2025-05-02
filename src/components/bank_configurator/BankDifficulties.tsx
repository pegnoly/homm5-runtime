import { invoke } from "@tauri-apps/api/core";
import { Segmented, Typography } from "antd";
import { useEffect, useState } from "react";
import BankStringProperty from "./utils";

export enum BankDifficultyType {
    Easy = "BANK_DIFFICULTY_EASY",
    Medium = "BANK_DIFFICULTY_MEDIUM",
    Hard = "BANK_DIFFICULTY_HARD",
    Critical = "BANK_DIFFICULTY_CRITICAL",
    Boss = "BANK_DIFFICULTY_BOSS"
}

export const difficultiesData = new Map<BankDifficultyType, string>([
    [BankDifficultyType.Easy, "Easy"],
    [BankDifficultyType.Medium, "Medium"],
    [BankDifficultyType.Hard, "Hard"],
    [BankDifficultyType.Critical, "Critical"],
    [BankDifficultyType.Boss, "Boss"]
]);

type BankDifficultyModel = {
    id: number,
    difficulty: BankDifficultyType,
    chance: number
}

function BankDifficulties(data: {bankId: number | undefined}) {
    const [difficulties, setDifficulties] = useState<BankDifficultyModel[]>([]);
    const [selectedDifficulty, setSelectedDifficulty] = useState<BankDifficultyModel | undefined>(undefined);

    useEffect(() => {
        if (data.bankId != undefined) {
            //console.log(`Bank id: ${data.bankId}`);
            invoke<BankDifficultyModel[]>("load_bank_difficulties", {bankId: data.bankId})
                .then((value) => {
                    //console.log(`Difficulties data: ${value}`)
                    setDifficulties(value);
                });
        }
    }, [data.bankId]);

    async function difficultyUpdated(updated: BankDifficultyModel) {
        const updatedDifficulties = difficulties.map((d) => {
            if (d.id == updated.id) {
                return updated;
            } else {
                return d;
            }
        });
        setDifficulties(updatedDifficulties);
        setSelectedDifficulty(updated);
    }

    return <div style={{display: 'flex', flexDirection: 'column', gap: 5, alignItems: 'center', height: '50%'}}>
        <Typography.Text style={{fontFamily: 'fantasy', fontStretch: 'expanded', fontSize: 20, color: 'darkorchid'}}>Difficulties</Typography.Text>
        <Segmented  
            options={difficulties.map((v) => ({value: v.id, label: difficultiesData.get(v.difficulty)}))}
            onChange={(id) => setSelectedDifficulty(difficulties.find(d => d.id == id))}
        />
        <DifficultyConfigurator selected={selectedDifficulty} updateCallback={difficultyUpdated}/>
    </div>
}

function DifficultyConfigurator(data: {selected: BankDifficultyModel | undefined, updateCallback: (updatedModel: BankDifficultyModel) => void}) {

    async function updateChance(newChance: string) {
        await invoke<number>("update_bank_difficulty_chance", {id: data.selected?.id, chance: newChance})
            .then((value) => data.updateCallback({...data.selected!, chance: value}));
    }

    return <>{
        data.selected != undefined ?
        <div style={{paddingTop: '5%'}}>
            <BankStringProperty text="Difficulty chance" initialValue={data.selected.chance} updateCallback={updateChance}/>
        </div> :
        <h1>Difficulty not selected</h1>
    }</>
}

export default BankDifficulties;