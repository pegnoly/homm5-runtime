export type Quest = {
    id: number,
    directory: string,
    name: string,
    desc: string,
    script_name: string,
    is_active: boolean,
    is_secondary: boolean,
    update_sources: QuestUpdateSources
}

export type QuestUpdateSourceVariableType = "string" | "number" | "bool" | "any"

export type QuestUpdateSourceVariable = {
    name: string,
    type: QuestUpdateSourceVariableType
}

export type QuestUpdateSource = {
    source: string,
    variables: QuestUpdateSourceVariable[],
    comment: string
}

export type QuestUpdateSources = {
    sources: QuestUpdateSource[]
}

export type OneOfQuestProgress = {
    text: string,
    start_value: number,
    count: number
}

export type QuestProgressType =
    | { type: "Default", data: string }
    | { type: "OneOf", data: OneOfQuestProgress }

export type QuestProgress = {
    id: number,
    number: number,
    text: string | null,
    one_of_progress: OneOfQuestProgress | null,
    concatenate: boolean
}