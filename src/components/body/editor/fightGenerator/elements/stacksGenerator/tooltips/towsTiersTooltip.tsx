import { Tooltip } from "@mantine/core";
import { ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";
import { useCountGenerationMode, useCurrentStackTiers, useCurrentStackTowns } from "../store";
import { StackCountGenerationType } from "../types";
import { TownType } from "../../../types";

function AverageTownsTiersTooltip({children, value} : EditablePropertyWrapperProps) {
    const generationMode = useCountGenerationMode();

    return (
    <>
    {
        generationMode == StackCountGenerationType.Raw ?
        children :
        <AverageTownsTiersTooltipRenderer power={value!}>
            {children}
        </AverageTownsTiersTooltipRenderer>
    }
    </>
    )
}

function useAverageTownsTiersCounts(towns: TownType[], tiers: number[], power: string) {
    return useQuery({
        queryKey: ["average_towns_tiers_counts", towns, tiers, power],
        queryFn: async() => {
            return invoke<number>("get_average_creatures_count", {power: parseInt(power), towns: towns, tiers: tiers});
        }
    })
}

function AverageTownsTiersTooltipRenderer({children, power} : {children: ReactNode, power: string}) {
    const towns = useCurrentStackTowns();
    const tiers = useCurrentStackTiers();

    const { data } = useAverageTownsTiersCounts(towns?.towns!, tiers?.tiers!, power);

    return (
        <Tooltip label={`Average creatures for given power is ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default AverageTownsTiersTooltip;