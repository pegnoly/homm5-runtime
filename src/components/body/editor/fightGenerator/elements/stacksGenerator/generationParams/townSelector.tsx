import {useMutation} from "@tanstack/react-query";
import {TownTypeExtended} from "../../../types";
import {useCurrentStackActions, useCurrentStackId, useCurrentStackTowns} from "../store";
import {FightGeneratorApi} from "../../../api";
import {MultiSelect} from "@mantine/core";

export type UpdateTownsPayload = {
    stackId: number,
    towns: TownTypeExtended []
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
                mutation.mutate({stackId: currentStackId!, towns: value.map(v => v as TownTypeExtended)});
            }}
            data={[
                {value: TownTypeExtended.TownAcademy, label: "Academy"},
                {value: TownTypeExtended.TownDungeon, label: "Dungeon"},
                {value: TownTypeExtended.TownHeaven, label: "Heaven"},
                {value: TownTypeExtended.TownInferno, label: "Inferno"},
                {value: TownTypeExtended.TownFortress, label: "Fortress"},
                {value: TownTypeExtended.TownPreserve, label: "Preserve"},
                {value: TownTypeExtended.TownNecromancy, label: "Necromancy"},
                {value: TownTypeExtended.TownStronghold, label: "Stronghold"},
                {value: TownTypeExtended.TownBastion, label: "Bastion"},
                {value: TownTypeExtended.TownSanctuary, label: "Sanctuary"},
                {value: TownTypeExtended.TownRenegades, label: "Renegades"},
                {value: TownTypeExtended.TownNoType, label: "Neutral"},
            ]}
        />
    )
}

export default TownSelector;