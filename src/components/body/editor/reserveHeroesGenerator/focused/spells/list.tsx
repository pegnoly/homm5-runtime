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
    <List>{spells?.spells.map((s, i) => (
        <div key={i} style={{display: 'flex', flexDirection: 'row'}}>
            <Text size="xs">{s}</Text>
            <ActionIcon size="xs" onClick={() => removeSpell(s)}>
                <IconTrash/>
            </ActionIcon>
        </div>
    ))}</List>
    )
}

export default ReserveHeroSpellsList;