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

export type SpeakerType =
    | "NoSpeaker"
    | { Hero: string }
    | { Creature: number }

export type Speaker = {
    id: number,
    name_override: string | null,
    color: string,
    speaker_type: SpeakerType,
    local_to_map: number | null
}

export function getSpeakerDisplayName(speaker: Speaker): string {
    if (speaker.name_override) {
        return speaker.name_override;
    }
    if (speaker.speaker_type === "NoSpeaker") {
        return "Без персонажа";
    }
    if ("Hero" in speaker.speaker_type) {
        return speaker.speaker_type.Hero;
    }
    return `Существо #${speaker.speaker_type.Creature}`;
}

export type VariantSpeakerIds = {
    ids: number[]
}

export type DialogVariant = {
    id: number,
    dialogId: number,
    step: number,
    label: string,
    speaker_ids: VariantSpeakerIds,
    text: string
}