import { WorkStage } from "./types";

export function getLabelColor(stage: WorkStage): string {
    switch (stage) {
        case WorkStage.Inactive:
            return "black"
            break;
        case WorkStage.Active:
            return "red"
            break;
        case WorkStage.Done:
            return "green"
            break
        default:
            return "black"
            break;
    }
}