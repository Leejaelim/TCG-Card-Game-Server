use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_main_character_health_point_data_response::GenerateOpponentMainCharacterHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_main_character_survival_data_response::GenerateOpponentMainCharacterSurvivalDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTargetingAttackToGameMainCharacterResponseForm {
    is_success: bool,
    player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>
}

impl DeployTargetingAttackToGameMainCharacterResponseForm {
    pub fn new(
        is_success: bool,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>
    ) -> Self {

        DeployTargetingAttackToGameMainCharacterResponseForm {
            is_success,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice
        }
    }

    pub fn from_response(
        generate_opponent_main_character_health_point_data_response: GenerateOpponentMainCharacterHealthPointDataResponse,
        generate_opponent_main_character_survival_data_response: GenerateOpponentMainCharacterSurvivalDataResponse
    ) -> DeployTargetingAttackToGameMainCharacterResponseForm {

        DeployTargetingAttackToGameMainCharacterResponseForm::new(
            true,
            generate_opponent_main_character_health_point_data_response
                .get_player_main_character_health_point_map_for_response().clone(),
            generate_opponent_main_character_survival_data_response
                .get_player_main_character_survival_map_for_response().clone())
    }

    pub fn default() -> DeployTargetingAttackToGameMainCharacterResponseForm {

        DeployTargetingAttackToGameMainCharacterResponseForm::new(false, HashMap::new(), HashMap::new())
    }
}