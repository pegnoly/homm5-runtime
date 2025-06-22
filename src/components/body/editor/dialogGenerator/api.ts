import { invoke } from "@tauri-apps/api/core";
import { Dialog, DialogSimple, DialogVariant, Speaker } from "./types";
import { DialogCreationPayload } from "./header/creator";
import { CreateSpeakerPayload } from "./header/speakerCreator";
import { GetVariantPayload } from "./body";
import { UpdateLabelsPayload } from "./body/sidePanel/labels";
import { SaveDialogVariantPayload } from "./body/sidePanel/save";

export class DialogGeneratorApi {
    static async loadDialogs(missionId: number): Promise<DialogSimple []> {
        return invoke<DialogSimple []>("load_dialogs", {missionId: missionId});
    }

    static async loadSpeakers(): Promise<Speaker []> {
        return invoke<Speaker []>("load_speakers");
    }

    static async createDialog(payload: DialogCreationPayload): Promise<Dialog> {
        return invoke<Dialog>("create_new_dialog", payload);
    }

    static async createSpeaker(payload: CreateSpeakerPayload): Promise<Speaker> {
        return invoke<Speaker>("create_speaker", payload);
    }

    static async loadDialog(id: number): Promise<Dialog> {
        return invoke<Dialog>("load_dialog", {id: id});
    }

    static async tryLoadVariant(payload: GetVariantPayload): Promise<DialogVariant> {
        return invoke<DialogVariant>("load_dialog_variant", payload);
    }

    static async updateLabels(payload: UpdateLabelsPayload): Promise<void> {
        return invoke("update_dialog_labels", payload);
    }

    static async saveVariant(payload: SaveDialogVariantPayload): Promise<void> {
        return invoke("save_dialog_variant", payload);
    }
}