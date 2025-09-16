use editor_tools::prelude::{BanksGeneratorRepo, DialogGeneratorRepo, FightGeneratorRepo, QuestGeneratorRepo, ReserveHeroCreatorRepo};
use homm5_scaner::prelude::ScanerService;
use services::dialog_generator::prelude::*;
use services::quest_creator::prelude::*;
use sheets_connector::service::SheetsConnectorService;
use std::path::PathBuf;
use tokio::sync::RwLock;
use utils::{LocalAppManager, RuntimeConfig, DataContainer, GlobalConfig, ModifiersConfig};
use crate::error::Error;

mod commands;
pub mod error;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Error> {
    let exe_path = std::env::current_exe().unwrap();
    let cfg_path = exe_path.parent().unwrap().join("cfg\\");

    let global_config = GlobalConfig::new(&cfg_path)?;
    let runtime_config = RuntimeConfig::new(&cfg_path)?;
    let modifiers_config = ModifiersConfig::new(&cfg_path)?;
    let data_container = DataContainer::new(&cfg_path)?;

    let db_path = cfg_path.join("runtime_database.db");
    if !db_path.exists() {
        std::fs::File::create(&db_path).unwrap();
    }

    let pool = sqlx::SqlitePool::connect(db_path.to_str().unwrap())
        .await
        .unwrap();
    //sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let quest_generator_repo = QuestGeneratorRepo::new(pool.clone());
    let dialog_generator_repo = DialogGeneratorRepo::new(pool.clone());
    let fight_generator_repo = FightGeneratorRepo::new(pool.clone());
    let reserve_hero_creator_repo = ReserveHeroCreatorRepo::new(pool.clone());
    let sheets_connector_repo = SheetsConnectorService::new(&global_config.auth_path).await.unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(LocalAppManager {
            base_config: RwLock::new(global_config),
            runtime_config: RwLock::new(runtime_config),
            modifiers_config: RwLock::new(modifiers_config)
        })
        .manage(quest_generator_repo)
        .manage(dialog_generator_repo)
        .manage(BanksGeneratorRepo::new(
            pool.clone(),
            PathBuf::from("D:/Homm5Dev/Mods/GOG/scripts/advmap/Banks/Data/"),
        ))
        .manage(fight_generator_repo)
        .manage(reserve_hero_creator_repo)
        .manage(ScanerService::new(pool.clone()))
        .manage(data_container)
        .manage(sheets_connector_repo)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::execute_scan,
            commands::run_game,
            commands::load_repackers,
            commands::repack,
            commands::load_maps,
            commands::load_current_map,
            commands::select_map,
            commands::apply_modifications,
            commands::create_hero,
            // quest commands
            load_quests,
            load_quest,
            pick_quest_directory,
            create_quest,
            load_quest_progress,
            save_progress,
            update_quest_directory,
            update_quest_script_name,
            update_quest_name,
            update_quest_desc,
            update_is_secondary,
            update_is_active,
            save_quest_text,
            add_quest_to_queue,
            // dialog commands
            load_dialogs,
            load_speakers,
            pick_dialog_directory,
            create_new_dialog,
            load_dialog,
            create_speaker,
            update_dialog_labels,
            add_dialog_speaker,
            load_dialog_variant,
            save_dialog_variant,
            generate_dialog,
            //banks
            services::banks_configurator::commands::get_all_banks,
            services::banks_configurator::commands::load_bank,
            services::banks_configurator::commands::load_difficulty,
            services::banks_configurator::commands::update_bank_difficulty_chance,
            services::banks_configurator::commands::create_bank_variant,
            services::banks_configurator::commands::update_bank_recharges_count,
            services::banks_configurator::commands::update_bank_recharge_timer,
            services::banks_configurator::commands::update_bank_morale_loss,
            services::banks_configurator::commands::update_bank_luck_loss,
            services::banks_configurator::commands::load_bank_variant,
            services::banks_configurator::commands::load_bank_variants,
            services::banks_configurator::commands::update_bank_variant_label,
            services::banks_configurator::commands::update_bank_variant_difficulty,
            services::banks_configurator::commands::create_creature_slot,
            services::banks_configurator::commands::load_creature_slots_ids,
            services::banks_configurator::commands::load_creature_slot,
            services::banks_configurator::commands::update_creature_slot_base_power,
            services::banks_configurator::commands::update_creature_slot_power_grow,
            services::banks_configurator::commands::update_creature_slot_tier,
            services::banks_configurator::commands::update_creature_slot_town,
            services::banks_configurator::commands::generate_banks_script,
            // hero generator
            services::fight_generator::commands::load_artifact_models,
            services::fight_generator::commands::load_creature_models,
            services::fight_generator::commands::load_abilities_models,
            services::fight_generator::commands::pick_hero_lua_generation_directory,
            services::fight_generator::commands::init_new_asset,
            services::fight_generator::commands::load_all_assets,
            services::fight_generator::commands::load_asset,
            services::fight_generator::commands::delete_asset,
            services::fight_generator::commands::try_load_artifacts_data_for_asset,
            services::fight_generator::commands::create_artifacts_data_for_asset,
            services::fight_generator::commands::update_artifacts_base_cost,
            services::fight_generator::commands::update_artifacts_cost_grow,
            services::fight_generator::commands::add_required_artifact,
            services::fight_generator::commands::remove_required_artifact,
            services::fight_generator::commands::add_optional_artifact,
            services::fight_generator::commands::remove_optional_artifact,
            services::fight_generator::commands::load_stacks_ids,
            services::fight_generator::commands::create_stack,
            services::fight_generator::commands::load_stack,
            services::fight_generator::commands::delete_stack,
            services::fight_generator::commands::update_stack_data,
            services::fight_generator::commands::update_stack_base_powers,
            services::fight_generator::commands::update_stack_powers_grow,
            services::fight_generator::commands::update_stack_concrete_count,
            services::fight_generator::commands::update_stack_concrete_creatures,
            services::fight_generator::commands::update_stack_towns,
            services::fight_generator::commands::update_stack_tiers,
            services::fight_generator::commands::update_stack_rules,
            services::fight_generator::commands::generate_current_hero_script,
            services::fight_generator::commands::load_stats_generation_elements,
            services::fight_generator::commands::add_stat_generation_element,
            services::fight_generator::commands::remove_stat_generation_element,
            services::fight_generator::commands::update_stat_generation_element_priority,
            services::fight_generator::commands::update_stat_generation_element_rule,
            services::fight_generator::commands::update_stat_generation_params,
            services::fight_generator::commands::get_average_creatures_count,
            services::fight_generator::commands::get_average_concrete_creatures_count,
            services::fight_generator::commands::get_average_artifacts_cost,
            services::fight_generator::commands::create_sheet_for_existing_asset,
            services::fight_generator::commands::pull_from_sheet,
            services::fight_generator::commands::push_to_sheet,
            //
            services::creature_generator::commands::save_generation_session,
            services::creature_generator::commands::generate_creatures,
            services::creature_generator::commands::pick_session_file,
            services::creature_generator::commands::load_session,
            //
            services::reserve_hero_creator::commands::load_heroes_data,
            services::reserve_hero_creator::commands::load_heroes,
            services::reserve_hero_creator::commands::init_new_hero,
            services::reserve_hero_creator::commands::load_base_skills,
            services::reserve_hero_creator::commands::load_existing_reserved_hero,
            services::reserve_hero_creator::commands::delete_reserved_hero,
            services::reserve_hero_creator::commands::add_skill,
            services::reserve_hero_creator::commands::remove_skill,
            services::reserve_hero_creator::commands::update_skill,
            services::reserve_hero_creator::commands::load_perks,
            services::reserve_hero_creator::commands::load_spells,
            services::reserve_hero_creator::commands::add_spell,
            services::reserve_hero_creator::commands::remove_spell,
            services::reserve_hero_creator::commands::update_favorite_enemies,
            //
            services::sheets_connector::commands::upload_to_sheets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
