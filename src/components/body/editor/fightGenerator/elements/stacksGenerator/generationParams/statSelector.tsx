import {
    ActionIcon, Group,
    List, Popover, PopoverDropdown, PopoverTarget,
    Text
} from "@mantine/core";
import {
    FightStackGeneratorStore,
} from "../store";
import {
    SelectionType,
    StatGenerationRule,
    StatGenerationType,
    StatRuleModel
} from "../types";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../../api";
import {IconMinus, IconPlus, IconTagPlus} from "@tabler/icons-react";
import {useState} from "react";

export type UpdateStatRulesPayload = {
    stackId: number,
    rules: StatRuleModel []
}

function FightAssetStackStatsData() {
    const id = FightStackGeneratorStore.useCurrentId();
    const selectionType = FightStackGeneratorStore.useSelectionType();
    const statsRules = FightStackGeneratorStore.useStatsRules();
    const actions = FightStackGeneratorStore.useActions();

    const [creatorActive, setCreatorActive] = useState<boolean>(false);

    if (selectionType != SelectionType.TierSlotBased || id == undefined || statsRules == undefined) {
        return null;
    }

    const removeRuleMutation = useMutation({
        mutationFn: async (payload: UpdateStatRulesPayload) => {
            return FightGeneratorApi.updateStackStatsRules(payload);
        },
        onSuccess: (_data, variables, _context) => {
            actions.setStatsRules(variables.rules);
        }
    })

    return (
    <>
        <Popover opened={creatorActive}>
            <PopoverTarget>
                <Group>
                    <ActionIcon onClick={() => setCreatorActive(!creatorActive)}>
                        <IconTagPlus/>
                    </ActionIcon>
                    <Text>Добавить зависимости от статов</Text>
                </Group>
            </PopoverTarget>
            <PopoverDropdown>
                <StackStatsCreator/>
            </PopoverDropdown>
        </Popover>
        <List>{statsRules?.rules!.map((m) => (
            <Group justify="space-between" pt={2} maw={200}>
                <Text>{m.rule}</Text>
                <Text>{m.param}</Text>
                <ActionIcon bg="red" radius={0} onClick={() => {
                    const updatedRules = statsRules?.rules
                        .filter((r) => !(r.rule == m.rule && r.param == m.param));
                    removeRuleMutation.mutate({stackId: id!, rules: updatedRules});
                }}>
                    <IconMinus/>
                </ActionIcon>
            </Group>
        ))}</List>
    </>
    )
}

function getMissingRules(existing: StatRuleModel[]): StatRuleModel[] {
    const existingKeys = new Set(
        existing.map((m) => `${m.rule}|${m.param}`)
    )

    const missing: StatRuleModel[] = [];
    for (const rule of Object.values(StatGenerationRule)) {
        for (const param of Object.values(StatGenerationType)) {
            const key = `${rule}|${param}`;
            if (!existingKeys.has(key)) {
                missing.push({ rule, param });
            }
        }
    }

    return missing;
}

function StackStatsCreator() {
    const currentRules = FightStackGeneratorStore.useStatsRules();
    const id = FightStackGeneratorStore.useCurrentId();
    const actions = FightStackGeneratorStore.useActions();

    const unusedRules = getMissingRules(currentRules?.rules == undefined ? [] : currentRules.rules)

    const addRuleMutation = useMutation({
        mutationFn: async (payload: UpdateStatRulesPayload) => {
            return FightGeneratorApi.updateStackStatsRules(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setStatsRules(variables.rules)
        }
    });

    return (
        <>
            <List>{unusedRules.map((r) => (
                <Group justify="space-between" pt={2}>
                    <Text>{r.rule}</Text>
                    <Text>{r.param}</Text>
                    <ActionIcon radius={0} bg="green" onClick={() => addRuleMutation.mutate({
                        stackId: id!,
                        rules: [...currentRules?.rules!, r]
                    })}>
                        <IconPlus/>
                    </ActionIcon>
                </Group>
            ))}</List>
        </>
    )
}

export default FightAssetStackStatsData;