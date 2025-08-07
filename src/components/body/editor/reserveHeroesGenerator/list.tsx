import { Link } from "react-router";
import { ReserveHeroesGenerator } from "./store";
import { List, Text } from "@mantine/core";
import { EditorState } from "@/stores/EditorStateStore";
import { ReservedHero } from "./types";

function ReserveHeroesList({heroes}: {heroes: ReservedHero[]}) {
    const currentId = ReserveHeroesGenerator.useCurrentId();

    return (
    <div style={{width: '10%', height: '100%', display: 'flex', justifyContent: 'center'}}>        
    {
        heroes == undefined ? null :
        <List>{heroes!.map((h, i) => (
            <Link 
                style={{textDecoration: 'none'}} 
                to={`/editor/${EditorState.ReserveHeroes}/focused/${h.id}`} key={i}>
                <div style={{backgroundColor: h.id == currentId ? 'greenyellow' : 'transparent'}}>
                    <Text c="dark" fw="bold">{h.name}</Text>
                </div>
            </Link>
        ))}</List>
    }
    </div>
    )
}

export default ReserveHeroesList;