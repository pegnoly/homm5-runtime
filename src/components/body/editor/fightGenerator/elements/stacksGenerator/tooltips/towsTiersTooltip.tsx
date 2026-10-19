import { Tooltip } from "@mantine/core";
import { ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";
import { FightStackGeneratorStore } from "../store";
import { CalculationType } from "../types";
import { TownTypeExtended } from "../../../types";

function AverageTownsTiersTooltip({children, value} : EditablePropertyWrapperProps) {
    const calculationType = FightStackGeneratorStore.useCalculationType();

    return (
    <>
    {
        calculationType == CalculationType.Raw ?
        children :
        <AverageTownsTiersTooltipRenderer power={value as string}>
            {children}
        </AverageTownsTiersTooltipRenderer>
    }
    </>
    )
}

function useAverageTownsTiersCounts(towns: TownTypeExtended[], tiers: number[], power: string) {
    return useQuery({
        queryKey: ["average_towns_tiers_counts", towns, tiers, power],
        queryFn: async() => {
            return invoke<number>("get_average_creatures_count", {power: parseInt(power), towns: towns, tiers: tiers});
        }
    })
}

function AverageTownsTiersTooltipRenderer({children, power} : {children: ReactNode, power: string}) {
    const towns = FightStackGeneratorStore.useTowns();
    const tiers = FightStackGeneratorStore.useTiers();

    const { data } = useAverageTownsTiersCounts(towns?.towns!, tiers?.tiers!, power);

    return (
        <Tooltip label={`Среднее число существ - ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default AverageTownsTiersTooltip;