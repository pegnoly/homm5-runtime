import { Button, Group, List, SegmentedControl, Text, TextInput } from '@mantine/core';
import styles from '../../styles.module.css';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { IconTrash } from '@tabler/icons-react';

function ReserveHeroesGeneratorLayout() {

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
    <div className={styles.editor_layout}>
        <SegmentedControl
            defaultValue={"1"}
            onChange={(value) => playerSelected(parseInt(value))}
            data={[
                {value: "1", label: "1"},
                {value: "2", label: "2"},
                {value: "3", label: "3"},
                {value: "4", label: "4"},
                {value: "5", label: "5"},
                {value: "6", label: "6"},
                {value: "7", label: "7"},
                {value: "8", label: "8"}
            ]}
        ></SegmentedControl>
        <div style={{width: '70%', paddingLeft: '10%'}}>
            <List>{heroes.map((h, i) => (
                <div style={{display: 'flex', flexDirection: 'row', gap: '5%', paddingTop: '2%'}}>
                    <Group>
                        <div key={i}>{h.replace("#xpointer(/AdvMapHeroShared)", "")}</div>
                        <Button
                            size='xs'
                            onClick={() => removeHero(h)}
                        >
                            <IconTrash/>
                        </Button>
                    </Group>
                </div>
            ))}</List>
            <Text>Enter new hero xdb</Text>
            <TextInput onChange={(e) => setNewHero(e.currentTarget.value)}/>
            <Button onClick={() => addHero(newHero)}>Add new hero</Button>
        </div>
    </div>
    )
}

export default ReserveHeroesGeneratorLayout;