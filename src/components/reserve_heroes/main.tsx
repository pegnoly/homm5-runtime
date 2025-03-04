import { DeleteOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input, List, Segmented, Typography } from "antd";
import { useEffect, useState } from "react";

function ReserveHeroesMain() {

    const [heroes, setHeroes] = useState<string[]>([])
    const [newHero, setNewHero] = useState<string>("")
    const [currentPlayer, setCurrentPlayer] = useState<number>(1)

    useEffect(() => {
        invoke<string[]>("load_existing_reserve_heroes", {player: currentPlayer})
            .then((hrs) => {
                console.log("Heroes: ", hrs)
                setHeroes(hrs)
            })
    }, [])

    function playerSelected(newPlayer: number) {
        setCurrentPlayer(newPlayer)
        invoke<string[]>("load_existing_reserve_heroes", {player: newPlayer})
            .then((hrs) => {
                setHeroes(hrs)
            })
    }

    function removeHero(hero: string) {
        let updatedHeroes = heroes.filter(h => h != hero)
        setHeroes(updatedHeroes)
        invoke("remove_reserve_hero", {hero: hero, player: currentPlayer})
    }

    function addHero(hero: string) {
        let updatedHeroes = [...heroes, hero.concat("#xpointer(/AdvMapHeroShared)")]
        setHeroes(updatedHeroes)
        invoke("add_reserve_hero", {hero: hero.concat("#xpointer(/AdvMapHeroShared)"), player: currentPlayer})
    }

    return (
        <>
            <Segmented
                defaultValue={1}
                onChange={playerSelected}
                options={[
                    {value: 1, label: "1"},
                    {value: 2, label: "2"},
                    {value: 3, label: "3"},
                    {value: 4, label: "4"},
                    {value: 5, label: "5"},
                    {value: 6, label: "6"},
                    {value: 7, label: "7"},
                    {value: 8, label: "8"}
                ]}
            ></Segmented>
            <List>{heroes.map((h, i) => (
                <div style={{display: 'flex', flexDirection: 'row', gap: 5}}>
                    <List.Item key={i}>{h.replace("#xpointer(/AdvMapHeroShared)", "")}</List.Item>
                    <Button
                        onClick={() => removeHero(h)} 
                        icon={<DeleteOutlined/>}
                    />
                </div>
            ))}</List>
            <Typography.Text>Enter new hero xdb</Typography.Text>
            <Input onChange={(e) => setNewHero(e.currentTarget.value)}/>
            <Button onClick={() => addHero(newHero)}>Add new hero</Button>
        </>
    )
}

export default ReserveHeroesMain;