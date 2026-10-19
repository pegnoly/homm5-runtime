import { Tooltip } from "@mantine/core";
import { ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";
import { FightStackGeneratorStore } from "../store";
import {CalculationType} from "@/components/body/editor/fightGenerator/elements/stacksGenerator/types.ts";

function SpecificCreaturesTooltip({children, value} : EditablePropertyWrapperProps) {
    const calculationType = FightStackGeneratorStore.useCalculationType();

    return (
    <>
    {
        calculationType == CalculationType.Raw ?
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
    const creatures = FightStackGeneratorStore.useSpecificCreatures();

    const { data } = useAverageConcreteCreaturesCounts(creatures?.ids!, power);

    return (
        <Tooltip label={`Среднее число существ для текущих значений - ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default SpecificCreaturesTooltip;