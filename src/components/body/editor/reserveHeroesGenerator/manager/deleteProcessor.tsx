import { Button } from "@mantine/core";
import { ReserveHeroesGenerator } from "../store";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";

function ReserveHeroDeleteProcessor({onDelete}: {onDelete: (value: number) => void}) {
    const id = ReserveHeroesGenerator.useCurrentId();
    const actions = ReserveHeroesGenerator.useActions();
    const navigate = useNavigate();

    console.log("Id: ", id);

    const mutation = useMutation({
        mutationFn: async() => {
            return invoke("delete_reserved_hero", {id: id});
        },
        onSuccess(_data, _variables, _context) {
            actions.unloadReserveHero();
            navigate({pathname: "/editor/ReserveHeroes"});
            onDelete(id!);
        },
    });

    return (
        <Button 
            radius={0} 
            disabled={id == undefined} 
            bg="red"
            onClick={() => mutation.mutate()}
        >
            Delete current hero
        </Button>
    )
}

export default ReserveHeroDeleteProcessor;