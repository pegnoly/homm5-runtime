export type Quest = {
    id: number,
    directory: string,
    name: string,
    desc: string,
    script_name: string,
    is_active: boolean,
    is_secondary: boolean
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