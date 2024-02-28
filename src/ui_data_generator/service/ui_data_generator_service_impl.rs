use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::ui_data_generator::repository::ui_data_generator_repository::UiDataGeneratorRepository;
use crate::ui_data_generator::repository::ui_data_generator_repository_impl::UiDataGeneratorRepositoryImpl;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::{GenerateMySpecificUnitEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::{GenerateMyFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_death_data_request::{GenerateOpponentSpecificUnitDeathDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_deck_card_list_data_request::{GenerateUseMyDeckCardListDataRequest};
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::{GenerateDrawMyDeckDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_field_energy_data_request::{GenerateOpponentFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_health_point_data_request::GenerateOpponentSpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_search_my_deck_data_request::{GenerateSearchMyDeckDataRequest};
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::{GenerateMySpecificUnitEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::{GenerateMyFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_death_data_response::{GenerateOpponentSpecificUnitDeathDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_deck_card_list_data_response::{GenerateUseMyDeckCardListDataResponse};
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::{GenerateDrawMyDeckDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::{GenerateOpponentFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_search_my_deck_data_response::{GenerateSearchMyDeckDataResponse};
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;

pub struct UiDataGeneratorServiceImpl {
    ui_data_generator_repository: Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>>,
    card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
}

impl UiDataGeneratorServiceImpl {
    pub fn new(
        ui_data_generator_repository: Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>>,
        card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    ) -> Self {

        UiDataGeneratorServiceImpl {
            ui_data_generator_repository,
            card_kind_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<UiDataGeneratorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<UiDataGeneratorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        UiDataGeneratorServiceImpl::new(
                            UiDataGeneratorRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl UiDataGeneratorService for UiDataGeneratorServiceImpl {

    // 내 턴에 핸드 사용 시 활용
    async fn generate_use_my_hand_card_data(
        &mut self,
        generate_use_my_hand_card_data_request: GenerateUseMyHandCardDataRequest)
        -> GenerateUseMyHandCardDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_my_hand_card_data()");

        let used_hand_card_id =
            generate_use_my_hand_card_data_request.get_used_hand_card_id();

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(&used_hand_card_id).await;

        drop(card_kind_repository_guard);

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_hand_card_data(
                used_hand_card_id,
                hand_card_kind_enum).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseMyHandCardDataResponse::new(
            info_tuple.0,
            info_tuple.1.get_player_hand_card_use_map().clone())
    }

    // 내 유닛
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        generate_my_specific_unit_energy_data_request: GenerateMySpecificUnitEnergyDataRequest)
        -> GenerateMySpecificUnitEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_specific_unit_energy_data()");

        let unit_index =
            generate_my_specific_unit_energy_data_request.get_unit_index();
        let updated_unit_energy_map =
            generate_my_specific_unit_energy_data_request.get_updated_unit_energy_map();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_my_specific_unit_energy_data(
                unit_index,
                updated_unit_energy_map.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateMySpecificUnitEnergyDataResponse::new(
            info_tuple.0.get_player_field_unit_energy_map().clone(),
            info_tuple.1.get_player_field_unit_energy_map().clone())
    }

    async fn generate_my_field_energy_data(
        &mut self,
        generate_my_field_energy_data_request: GenerateMyFieldEnergyDataRequest)
        -> GenerateMyFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_my_field_energy_data()");

        let remaining_field_energy =
            generate_my_field_energy_data_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_field_energy_data(
                remaining_field_energy).await;

        drop(ui_data_generator_repository_guard);

        GenerateMyFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_energy_map().clone())
    }

    async fn generate_use_my_deck_card_list_data(
        &mut self,
        generate_use_my_deck_card_list_data_request: GenerateUseMyDeckCardListDataRequest)
        -> GenerateUseMyDeckCardListDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_use_my_deck_card_list_data()");

        let deck_card_id_list =
            generate_use_my_deck_card_list_data_request.get_deck_card_id_list();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_use_my_deck_card_list_data(
                deck_card_id_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateUseMyDeckCardListDataResponse::new(
            info_tuple.0.get_player_deck_card_use_list_map().clone(),
            info_tuple.1.get_player_deck_card_use_list_map().clone())
    }

    async fn generate_draw_my_deck_data(
        &mut self,
        generate_draw_my_deck_data_request: GenerateDrawMyDeckDataRequest)
        -> GenerateDrawMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_draw_my_deck_data()");

        let drawn_card_list =
            generate_draw_my_deck_data_request.get_drawn_card_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_draw_my_deck_data(
                drawn_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateDrawMyDeckDataResponse::new(
            info_tuple.0.get_player_drawn_card_list_map().clone(),
            info_tuple.1.get_player_draw_count_map().clone())
    }

    async fn generate_search_my_deck_data(
        &mut self,
        generate_search_my_deck_data_request: GenerateSearchMyDeckDataRequest)
        -> GenerateSearchMyDeckDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_search_my_deck_data()");

        let found_card_list =
            generate_search_my_deck_data_request.get_found_card_list().clone();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_search_my_deck_data(
                found_card_list.clone()).await;

        drop(ui_data_generator_repository_guard);

        GenerateSearchMyDeckDataResponse::new(
            info_tuple.0.get_player_search_card_list_map().clone(),
            info_tuple.1.get_player_search_count_map().clone())
    }

    async fn generate_opponent_field_energy_data(
        &mut self,
        generate_opponent_field_energy_data_request: GenerateOpponentFieldEnergyDataRequest)
        -> GenerateOpponentFieldEnergyDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_field_energy_data()");

        let remaining_field_energy =
            generate_opponent_field_energy_data_request.get_remaining_field_energy();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_field_energy_data(
                remaining_field_energy).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentFieldEnergyDataResponse::new(
            info_tuple.0.get_player_field_energy_map().clone(),
            info_tuple.1.get_player_field_energy_map().clone())
    }

    async fn generate_opponent_specific_unit_death_data(
        &mut self,
        generate_opponent_specific_unit_death_data_request: GenerateOpponentSpecificUnitDeathDataRequest)
        -> GenerateOpponentSpecificUnitDeathDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_death_data()");

        let opponent_dead_unit_index =
            generate_opponent_specific_unit_death_data_request.get_dead_unit_index();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_death_data(
                opponent_dead_unit_index).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitDeathDataResponse::new(
            info_tuple.0.get_player_field_unit_death_map().clone(),
            info_tuple.1.get_player_field_unit_death_map().clone())
    }

    async fn generate_opponent_specific_unit_health_point_data(
        &mut self,
        generate_opponent_specific_unit_health_point_data_request: GenerateOpponentSpecificUnitHealthPointDataRequest)
        -> GenerateOpponentSpecificUnitHealthPointDataResponse {

        println!("UiDataGeneratorServiceImpl: generate_opponent_specific_unit_health_point_data()");

        let opponent_unit_index =
            generate_opponent_specific_unit_health_point_data_request.get_opponent_unit_index();
        let opponent_unit_updated_health_point =
            generate_opponent_specific_unit_health_point_data_request.get_opponent_unit_updated_health_point();

        let mut ui_data_generator_repository_guard =
            self.ui_data_generator_repository.lock().await;

        let info_tuple =
            ui_data_generator_repository_guard.generate_opponent_specific_unit_health_point_data(
                opponent_unit_index,
                opponent_unit_updated_health_point).await;

        drop(ui_data_generator_repository_guard);

        GenerateOpponentSpecificUnitHealthPointDataResponse::new(
            info_tuple.0.get_player_field_unit_health_point_map().clone(),
            info_tuple.1.get_player_field_unit_health_point_map().clone())
    }
}