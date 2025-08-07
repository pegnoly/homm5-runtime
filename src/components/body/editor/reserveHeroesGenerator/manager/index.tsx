import { ButtonGroup } from "@mantine/core";
import { ReservedHero } from "../types";
import ReservedHeroCreator from "./creator";
import ReserveHeroDeleteProcessor from "./deleteProcessor";

function ReserveHeroesManager({player, onCreate, onDelete}: {
    player: number
    onCreate: (value: ReservedHero) => void,
    onDelete: (value: number) => void
}) {
    return (
        <ButtonGroup>
            <ReservedHeroCreator player={player} onCreate={onCreate}/>
            <ReserveHeroDeleteProcessor onDelete={onDelete}/>
        </ButtonGroup>
    )
}

export default ReserveHeroesManager;