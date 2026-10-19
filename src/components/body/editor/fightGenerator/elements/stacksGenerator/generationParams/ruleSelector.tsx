import { useMutation } from "@tanstack/react-query";
import { FightStackGeneratorStore } from "../store";
import { StackGenerationParam } from "../types";
import { FightGeneratorApi } from "../../../api";
import { MultiSelect } from "@mantine/core";

export type UpdateRulesPayload = {
    stackId: number,
    rules: StackGenerationParam []
}

function AssetStackRuleSelector() {
    const id = FightStackGeneratorStore.useCurrentId();
    const rules = FightStackGeneratorStore.useGenerationRules();
    const actions = FightStackGeneratorStore.useActions();

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
            label="Укажите правила"
            value={rules?.rules}
            onChange={(value) => {
                mutation.mutate({stackId: id!, rules: value.map(v => v as StackGenerationParam)});
            }}
            data={[
                {value: StackGenerationParam.Caster, label: "Только кастеры"},
                {value: StackGenerationParam.Shooter, label: "Только стрелки"},
                {value: StackGenerationParam.Generatable, label: "Только генерируемые"},
                {value: StackGenerationParam.UpgradeOnly, label: "Только апгрейды"},
            ]}
        />
    )
}

export default AssetStackRuleSelector;