import { invoke } from "@tauri-apps/api/core";
import { UpdateTownsPayload } from "./elements/stacksGenerator/generationParams/townSelector";
import { UpdateTiersPayload } from "./elements/stacksGenerator/generationParams/tiersSelector";
import { ArmyGenerationStatElement, FightAssetStackModel, StatGenerationRule, StatGenerationType } from "./elements/stacksGenerator/types";
import { CreateStackPayload } from "./elements/stacksGenerator/stackCreator";
import { UpdateRulesPayload } from "./elements/stacksGenerator/generationParams/ruleSelector";
import { UpdateConcreteCreaturesPayload } from "./elements/stacksGenerator/generationParams";

export class FightGeneratorApi {
    static async loadStack(stackId: number): Promise<FightAssetStackModel> {
        return invoke<FightAssetStackModel>("load_stack", {stackId: stackId});
    }

    static async createStack(payload: CreateStackPayload): Promise<number> {
        return invoke<number>("create_stack", payload);
    }

    static async updateStackTowns(payload: UpdateTownsPayload): Promise<void> {
        return invoke("update_stack_towns", payload);
    }

    static async updateStackConcreteCreatures(payload: UpdateConcreteCreaturesPayload): Promise<void> {
        return invoke("update_stack_concrete_creatures", payload);
    }

    static async updateStackTiers(payload: UpdateTiersPayload): Promise<void> {
        return invoke("update_stack_tiers", payload);
    }

    static async updateStackRules(payload: UpdateRulesPayload): Promise<void> {
        return invoke("update_stack_rules", payload);
    }

    static async loadStatParamElements(stackId: number): Promise<ArmyGenerationStatElement[]> {
        return invoke("load_stats_generation_elements", {stackId: stackId});
    }

    static async createStatParamElement(stackId: number): Promise<ArmyGenerationStatElement> {
        return invoke("add_stat_generation_element", {stackId: stackId, rule: StatGenerationRule.MaxBy});
    }

    static async removeStatParamElement(elementId: number): Promise<void> {
        return invoke("remove_stat_generation_element", {elementId: elementId});
    }

    static async updateStatParamElementPriority(elementId: number, priority: number): Promise<void> {
        return invoke("update_stat_generation_element_priority", {elementId: elementId, priority: priority});
    }

    static async updateStatParamElementRule(elementId: number, rule: StatGenerationRule): Promise<void> {
        //console.log("Element id: ", elementId, " rule: ", rule);
        return invoke("update_stat_generation_element_rule", {elementId: elementId, rule: rule});
    }

    static async updateStatParamElementStats(elementId: number, stats: StatGenerationType []): Promise<void> {
        //console.log("Element id: ", elementId, " stats: ", stats);
        return invoke("update_stat_generation_params", {elementId: elementId, params: stats});
    }
}