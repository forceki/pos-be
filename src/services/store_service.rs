use crate::utils::pagination::PaginationMeta;
use crate::{dtos::pagination_dto::PaginationQuery, repository::store_repository::StoreRepository};
use crate::dtos::store_dto::CreateStoreDto;
use crate::models::store_model::Store;
use crate::utils::text_utils; // Pastikan kamu punya slug generator
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use uuid::Uuid;
use chrono::Utc;

pub struct StoreService {
    repo: StoreRepository,
}

impl StoreService {
    pub fn new(repo: StoreRepository) -> Self {
        Self { repo }
    }

    pub async fn create_store(&self, dto: CreateStoreDto) -> Result<String, actix_web::Error> {

        if self.repo.exists_by_code(&dto.code).await.map_err(|e| ErrorInternalServerError(e))? {
            return Err(ErrorBadRequest(format!("Store code '{}' already exists", dto.code)));
        }

        let slug = text_utils::to_slug(&dto.name);

        let is_warehouse_val = dto.is_warehouse as i8;
        let new_store = Store {
            id: Uuid::new_v4().to_string(),
            company_id: "".to_string(), 
            code: dto.code,
            name: dto.name,
            address: dto.address,
            store_number_phone: dto.store_number_phone,
            slug,
            is_warehouse: is_warehouse_val, 
            status: 1, 
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repo.create(&new_store).await
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(new_store.id)
    }

    pub async fn get_all_stores(&self, query: PaginationQuery) ->  Result<(Vec<Store>, PaginationMeta), actix_web::Error> {
        let page = query.get_page();
        let limit = query.get_limit();
        let offset = query.get_offset();

        let (data_db, total_items) = self.repo.get_all(limit, offset)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

       let data_dto: Vec<Store> = data_db
            .into_iter()
            .map(Store::from)
            .collect();

        let meta = PaginationMeta::new(page, limit, total_items);

        return Ok((data_dto, meta));
    }
}