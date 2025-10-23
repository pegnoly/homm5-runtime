import { Tooltip } from "@mantine/core";
import { ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { EditablePropertyWrapperProps } from "@/components/common/editableProperty";
import { useOptionalArtifacts } from "../store";
import { ArtifactSlotType } from "../types";

function AverageArtifactsCostTooltip({children} : EditablePropertyWrapperProps) {
    return (
    <>
        <AverageArtifactsCostTooltipRenderer>
            {children}
        </AverageArtifactsCostTooltipRenderer>
    </>
    )
}

function useAverageArtifactsCost(artifacts: Record<ArtifactSlotType, number[]>) {
    return useQuery({
        queryKey: ["average_artifacts_cost", artifacts],
        queryFn: async() => {
            return invoke<number>("get_average_artifacts_cost", {artifacts: artifacts});
        }
    })
}

function AverageArtifactsCostTooltipRenderer({children} : {children: ReactNode}) {
    const artifacts = useOptionalArtifacts();
    
    const { data } = useAverageArtifactsCost(artifacts?.values!);

    return (
        <Tooltip label={`Current average artifacts cost is ${data}`}>
            {children}
        </Tooltip> 
    )            
}

export default AverageArtifactsCostTooltip;