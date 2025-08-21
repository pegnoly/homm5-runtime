import { ActionIcon, List, Text } from "@mantine/core";
import { ReserveHeroesGenerator } from "../../store";
import { IconTrash } from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";

function ReserveHeroSpellsList() {
    const id = ReserveHeroesGenerator.useCurrentId();
    const spells = ReserveHeroesGenerator.useSpells();
    const actions = ReserveHeroesGenerator.useActions();

    async function removeSpell(gameId: string) {
        await invoke("remove_spell", {id: id, spell: gameId});
        actions.removeSpell(gameId);
    }

    return (
    <div style={{display: 'flex', justifyContent: 'center', paddingTop: '2%', overflow: 'auto'}}>
        <List>{spells?.spells.map((s, i) => (
            <div key={i} style={{display: 'flex', flexDirection: 'row', gap: '5%', justifyContent: 'space-around', alignItems: 'center'}}>
                <Text style={{fontFamily: 'cursive', fontSize: 11}}>{s}</Text>
                <ActionIcon size="xs" onClick={() => removeSpell(s)}>
                    <IconTrash/>
                </ActionIcon>
            </div>
        ))}</List>
    </div>
    )
}

export default ReserveHeroSpellsList;