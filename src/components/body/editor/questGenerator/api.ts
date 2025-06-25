import { invoke } from "@tauri-apps/api/core";
import { Quest, QuestProgress } from "./types";
import { CreateQuestPayload } from "./header/creator";
import { UpdateQuestPayloadBase } from "./body/data";
import { SaveQuestProgressPayload } from "./body/progress";

export class QuestGeneratorApi {
    static async loadQuests(): Promise<Quest[]> {
        return invoke<Quest[]>("load_quests");
    }

    static async loadQuest(id: number): Promise<Quest | null> {
        return invoke<Quest | null>("load_quest", {id: id});
    }

    static async createQuest(payload: CreateQuestPayload): Promise<Quest> {
        return invoke<Quest>("create_quest", payload);
    }

    static async updateQuestName(payload: UpdateQuestPayloadBase & {name: string}): Promise<void> {
        return invoke("update_quest_name", payload);
    }

    static async updateQuestDesc(payload: UpdateQuestPayloadBase & {desc: string}): Promise<void> {
        return invoke("update_quest_desc", payload);
    }

    static async updateQuestScriptName(payload: UpdateQuestPayloadBase & {scriptName: string}): Promise<void> {
        return invoke("update_quest_script_name", payload);
    }

    static async updateQuestDirectory(payload: UpdateQuestPayloadBase & {directory: string}): Promise<void> {
        return invoke("update_quest_directory", payload);
    }

    static async updateQuestIsSecondary(payload: UpdateQuestPayloadBase & {isSecondary: boolean}): Promise<void> {
        return invoke("update_is_secondary", payload);
    }

    static async updateQuestIsActive(payload: UpdateQuestPayloadBase & {isActive: boolean}): Promise<void> {
        return invoke("update_is_active", payload);
    }

    static async loadProgress(questId: number, progressNumber: number): Promise<QuestProgress> {
        return invoke<QuestProgress>("load_quest_progress", {questId: questId, number: progressNumber});
    }

    static async saveProgress(payload: SaveQuestProgressPayload): Promise<void> {
        return invoke("save_progress", payload);
    }
}