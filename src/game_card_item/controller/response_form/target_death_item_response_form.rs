use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_death_data_response::GenerateOpponentSpecificUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDeathItemResponseForm {
    is_success: bool,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl TargetDeathItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        TargetDeathItemResponseForm {
            is_success,
            player_field_unit_health_point_map,
            player_field_unit_death_map
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_opponent_specific_unit_health_point_data_response: GenerateOpponentSpecificUnitHealthPointDataResponse,
        generate_opponent_specific_unit_death_data_response: GenerateOpponentSpecificUnitDeathDataResponse
    ) -> TargetDeathItemResponseForm {

        TargetDeathItemResponseForm::new(
            generate_use_my_hand_card_data_response
                .is_success_for_response(),
            generate_opponent_specific_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone())
    }

    pub fn default() -> TargetDeathItemResponseForm {

        TargetDeathItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new())
    }
}