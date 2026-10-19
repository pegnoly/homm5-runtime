import { invoke } from "@tauri-apps/api/core";
import { UpdateTownsPayload } from "./elements/stacksGenerator/generationParams/townSelector";
import { UpdateTiersPayload } from "./elements/stacksGenerator/generationParams/tiersSelector";
import { FightAssetStackModel } from "./elements/stacksGenerator/types";
import { CreateStackPayload } from "./elements/stacksGenerator/stackCreator";
import { UpdateRulesPayload } from "./elements/stacksGenerator/generationParams/ruleSelector";
import { UpdateConcreteCreaturesPayload } from "./elements/stacksGenerator/generationParams";
import { FightAssetArtifactsModel } from "./elements/artifactsGenerator/types";
import { CreateArtifactsAssetPayload } from "./elements/artifactsGenerator/creator";
import { AddRequiredArtifactPayload, RemoveRequiredArtifactPayload } from "./elements/artifactsGenerator/required";
import { AddOptionalArtifactPayload, RemoveOptionalArtifactPayload } from "./elements/artifactsGenerator/optional";
import { UpdateStackDataPayload } from "./elements/stacksGenerator/data";
import {
    UpdateStatRulesPayload
} from "@/components/body/editor/fightGenerator/elements/stacksGenerator/generationParams/statSelector.tsx";

export class FightGeneratorApi {
    static async loadStack(stackId: number): Promise<FightAssetStackModel> {
        return invoke<FightAssetStackModel>("load_stack", {stackId: stackId});
    }

    static async deleteAsset(assetId: number): Promise<void> {
        return invoke("delete_asset", {id: assetId});
    }

    static async createStack(payload: CreateStackPayload): Promise<number> {
        return invoke<number>("create_stack", payload);
    }

    static async deleteStack(stackId: number): Promise<void> {
        return invoke("delete_stack", {stackId: stackId});
    }

    static async updateStackData(payload: UpdateStackDataPayload): Promise<void> {
        return invoke("update_stack_data", payload);
    }

    static async updateStackTowns(payload: UpdateTownsPayload): Promise<void> {
        return invoke("update_stack_towns", payload);
    }

    static async updateStackConcreteCreatures(payload: UpdateConcreteCreaturesPayload): Promise<void> {
        return invoke("update_stack_specific_creatures", payload);
    }

    static async updateStackTiers(payload: UpdateTiersPayload): Promise<void> {
        return invoke("update_stack_tiers", payload);
    }

    static async updateStackRules(payload: UpdateRulesPayload): Promise<void> {
        return invoke("update_stack_rules", payload);
    }

    static async updateStackStatsRules(payload: UpdateStatRulesPayload): Promise<void> {
        return invoke("update_stack_stat_rules", payload);
    }

    static async createArtifactsAsset(payload: CreateArtifactsAssetPayload): Promise<FightAssetArtifactsModel> {
        return invoke<FightAssetArtifactsModel>("create_artifacts_data_for_asset", payload);
    }

    static async tryLoadArtifactAsset(assetId: number): Promise<FightAssetArtifactsModel | null> {
        return invoke<FightAssetArtifactsModel | null>("try_load_artifacts_data_for_asset", {assetId: assetId});
    }

    static async addRequiredArtifact(payload: AddRequiredArtifactPayload): Promise<void> {
        return invoke("add_required_artifact", payload);
    }

    static async removeRequiredArtifact(payload: RemoveRequiredArtifactPayload): Promise<void> {
        return invoke("remove_required_artifact", payload);
    }

    static async addOptionalArtifact(payload: AddOptionalArtifactPayload): Promise<void> {
        return invoke("add_optional_artifact", payload);
    }

    static async removeOptionalArtifact(payload: RemoveOptionalArtifactPayload): Promise<void> {
        return invoke("remove_optional_artifact", payload);
    }
}