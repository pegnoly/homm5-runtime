import { useMutation } from "@tanstack/react-query";
import { TownType } from "../../../types";
import { useCurrentStackActions, useCurrentStackId, useCurrentStackTowns } from "../store";
import { FightGeneratorApi } from "../../../api";
import { MultiSelect } from "@mantine/core";

export type UpdateTownsPayload = {
    stackId: number,
    towns: TownType []
}

function TownSelector() {
    const currentStackId = useCurrentStackId();
    const towns = useCurrentStackTowns();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(payload: UpdateTownsPayload) => {
            return FightGeneratorApi.updateStackTowns(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setTowns(variables.towns)
        },
    })

    return (
        <MultiSelect
            disabled={mutation.isPending}
            radius={0}
            miw={100}
            size="xs"
            label="Select towns"
            value={towns?.towns}
            onChange={(value) => {
                mutation.mutate({stackId: currentStackId!, towns: value.map(v => v as TownType)});
            }}
            data={[
                {value: TownType.TownAcademy, label: "Academy"}, 
                {value: TownType.TownDungeon, label: "Dungeon"},
                {value: TownType.TownHeaven, label: "Heaven"},
                {value: TownType.TownInferno, label: "Inferno"},
                {value: TownType.TownFortress, label: "Fortress"},
                {value: TownType.TownPreserve, label: "Preserve"},
                {value: TownType.TownNecromancy, label: "Necromancy"},
                {value: TownType.TownStronghold, label: "Stronghold"},
                {value: TownType.TownNoType, label: "Neutral"},
            ]}
        />
    )
}

export default TownSelector;