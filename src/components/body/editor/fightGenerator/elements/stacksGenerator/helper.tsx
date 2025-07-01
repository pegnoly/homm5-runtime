import { Tooltip } from "@mantine/core";
import { useCountGenerationMode, useCurrentStackTiers, useCurrentStackTowns } from "./store";
import { StackCountGenerationType } from "./types";
import { ReactNode } from "react";
import { TownType } from "../../types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";

function AverageCreaturesTooltip({children, value} : EditablePropertyWrapperProps) {
    const generationMode = useCountGenerationMode();

    return (
    <>
    {
        generationMode == StackCountGenerationType.Raw ?
        children :
        <AverageCreaturesTooltipRenderer power={value}>
            {children}
        </AverageCreaturesTooltipRenderer>
    }
    </>
    )
}

function useAverageCounts(towns: TownType[], tiers: number[], power: string) {
    return useQuery({
        queryKey: ["average_counts", towns, tiers, power],
        queryFn: async() => {
            return invoke<number>("get_average_creatures_count", {power: parseInt(power), towns: towns, tiers: tiers});
        }
    })
}

function AverageCreaturesTooltipRenderer({children, power} : {children: ReactNode, power: string}) {
    const towns = useCurrentStackTowns();
    const tiers = useCurrentStackTiers();

    const { data } = useAverageCounts(towns?.towns!, tiers?.tiers!, power);

    return (
        <Tooltip label={`Average creatures for given power is ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default AverageCreaturesTooltip;