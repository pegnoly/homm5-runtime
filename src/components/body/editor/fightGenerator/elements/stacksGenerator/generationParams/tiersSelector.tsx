import { MultiSelect } from "@mantine/core";
import { useCurrentStackActions, useCurrentStackId, useCurrentStackTiers } from "../store";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../../api";

export type UpdateTiersPayload = {
    stackId: number,
    tiers: number []
}

function TiersSelector() {
    const currentStackId = useCurrentStackId();
    const tiers = useCurrentStackTiers();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(payload: UpdateTiersPayload) => {
            return FightGeneratorApi.updateStackTiers(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setTiers(variables.tiers)
        },
    })

    return (
        <MultiSelect
            radius={0}
            miw={100}
            size="xs"
            label="Select tiers"
            value={tiers!.tiers.map(v => v.toString())}
            onChange={(value) => {
                mutation.mutate({stackId: currentStackId!, tiers: value.map(t => parseInt(t))})
            }}
            data={[
                {value: "1", label: "Tier 1"},
                {value: "2", label: "Tier 2"},
                {value: "3", label: "Tier 3"},
                {value: "4", label: "Tier 4"},
                {value: "5", label: "Tier 5"},
                {value: "6", label: "Tier 6"},
                {value: "7", label: "Tier 7"},
            ]}
        />
    )
}

export default TiersSelector;