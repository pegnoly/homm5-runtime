import { Link } from "react-router";
import { ReserveHeroesGenerator } from "./store";
import { List, Text } from "@mantine/core";

function ReserveHeroesList() {
    const heroes = ReserveHeroesGenerator.useHeroes();

    return (
    <div style={{width: '10%', height: '100%', display: 'flex', justifyContent: 'center'}}>        
    {
        heroes == undefined ? null :
        <List>{heroes!.map((h, i) => (
            <Link to={`focused/${h.id}`} key={i}>
                <div>
                    <Text>{h.name}</Text>
                </div>
            </Link>
        ))}</List>
    }
    </div>
    )
}

export default ReserveHeroesList;