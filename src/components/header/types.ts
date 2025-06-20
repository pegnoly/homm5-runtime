export enum WorkStage {
    Inactive,
    Active,
    Done
}

export const repackStagesLabels = new Map<WorkStage, string>([
    [WorkStage.Inactive, "Inactive"],
    [WorkStage.Active, "In progress"],
    [WorkStage.Done, "Done"]
])