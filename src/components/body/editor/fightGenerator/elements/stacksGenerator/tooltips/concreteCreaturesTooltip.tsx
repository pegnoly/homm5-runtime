import { Tooltip } from "@mantine/core";
import { ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";
import { useConcreteCreatures, useCountGenerationMode } from "../store";
import { StackCountGenerationType } from "../types";

function ConcreteCreaturesTooltip({children, value} : EditablePropertyWrapperProps) {
    const generationMode = useCountGenerationMode();

    return (
    <>
    {
        generationMode == StackCountGenerationType.Raw ?
        children :
        <ConcreteCreaturesTooltipRenderer power={value as string}>
            {children}
        </ConcreteCreaturesTooltipRenderer>
    }
    </>
    )
}

function useAverageConcreteCreaturesCounts(creatures: number[], power: string) {
    return useQuery({
        queryKey: ["average_concrete_creatures_counts", creatures, power],
        queryFn: async() => {
            return invoke<number>("get_average_concrete_creatures_count", {power: parseInt(power), creatures: creatures});
        }
    })
}

function ConcreteCreaturesTooltipRenderer({children, power} : {children: ReactNode, power: string}) {
    const creatures = useConcreteCreatures();

    const { data } = useAverageConcreteCreaturesCounts(creatures?.ids!, power);

    return (
        <Tooltip label={`Average creatures for given power is ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default ConcreteCreaturesTooltip;