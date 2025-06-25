export type Quest = {
    id: number,
    directory: string,
    name: string,
    desc: string,
    script_name: string,
    is_active: boolean,
    is_secondary: boolean
}

export type QuestProgress = {
    id: number,
    text: string,
    concatenate: boolean
}