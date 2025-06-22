export type DialogSimple = {
    id: number,
    name: string
}

type SpeakersIds = {
    ids: number []
}

type Labels = {
    labels: string []
}

export type Dialog = {
    id: number,
    name: string,
    script_name: string,
    directory: string,
    speakers_ids: SpeakersIds,
    labels: Labels
}

export enum SpeakerType {
    Hero = "SPEAKER_TYPE_HERO",
    Creature = "SPEAKER_TYPE_CREATURE"
}

export type Speaker = {
    id: number,
    name: string
}

export type DialogVariant = {
    id: number,
    dialogId: number,
    step: number,
    label: string,
    speaker: number | null,
    text: string
}