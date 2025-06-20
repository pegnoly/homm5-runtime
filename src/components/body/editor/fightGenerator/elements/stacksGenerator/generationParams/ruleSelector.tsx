import { useMutation } from "@tanstack/react-query";
import { useCurrentStackActions, useCurrentStackId, useCurrentStackRules } from "../store";
import { StackGenerationParam } from "../types";
import { FightGeneratorApi } from "../../../api";
import { MultiSelect } from "@mantine/core";

export type UpdateRulesPayload = {
    stackId: number,
    rules: StackGenerationParam []
}

function AssetStackRuleSelector() {
    const currentStackId = useCurrentStackId();
    const rules = useCurrentStackRules();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(payload: UpdateRulesPayload) => {
            return FightGeneratorApi.updateStackRules(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setRules(variables.rules)
        },
    })

    return (
        <MultiSelect
            disabled={mutation.isPending}
            radius={0}
            miw={100}
            size="xs"
            label="Select rules"
            value={rules?.params}
            onChange={(value) => {
                mutation.mutate({stackId: currentStackId!, rules: value.map(v => v as StackGenerationParam)});
            }}
            data={[
                {value: StackGenerationParam.Caster, label: "Casters only"},
                {value: StackGenerationParam.Shooter, label: "Shooters only"},
                {value: StackGenerationParam.Generatable, label: "Generatable only"},
                {value: StackGenerationParam.UpgradeOnly, label: "Upgrades only"},
            ]}
        />
    )
}

export default AssetStackRuleSelector;