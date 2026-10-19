import {
    ActionIcon,
    List, Popover, PopoverDropdown, PopoverTarget, Select,
    Text
} from "@mantine/core";
import {
    FightStackGeneratorStore,
} from "../store";
import {
    SelectionType,
    StatRuleModel
} from "../types";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../../api";
import {IconRowRemove, IconTagPlus} from "@tabler/icons-react";
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
        <List>{statsRules?.rules!.map((m) => (
           <div style={{display: 'flex', flexDirection: 'row'}}>
               <Text>{m.rule}</Text>
               <Text>{m.param}</Text>
               <ActionIcon radius={0} onClick={() => {
                   const updatedRules = statsRules?.rules
                       .filter((r) => !(r.rule == m.rule && r.param == m.param));
                   removeRuleMutation.mutate({stackId: id!, rules: updatedRules});
               }}>
                   <IconRowRemove/>
               </ActionIcon>
           </div>
        ))}</List>
        <Popover opened={creatorActive}>
            <PopoverTarget>
                <ActionIcon onClick={() => setCreatorActive(!creatorActive)}>
                    <IconTagPlus/>
                </ActionIcon>
            </PopoverTarget>
            <PopoverDropdown>

            </PopoverDropdown>
        </Popover>
    </>
    )
}

// function StackStatsCreator() {
//     const currentRules = FightStackGeneratorStore.useStatsRules();
//
// }

export default FightAssetStackStatsData;