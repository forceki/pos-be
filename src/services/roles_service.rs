use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    dtos::{pagination_dto::PaginationQuery, roles_dto::{CreateRolesDTO, RolesResponseDTO}}, 
    models::roles_model::Roles, 
    repository::roles_repository::RolesRepository, utils::pagination::PaginationMeta
};

pub struct RolesService{
    repo: RolesRepository
}

impl RolesService{
    pub fn new(repo: RolesRepository) -> Self {
        RolesService{repo}
    }

    pub async fn create(&self, body: CreateRolesDTO) -> Result<String, actix_web::Error> {
        let role = Roles {
            id: Uuid::new_v4().to_string(),
            name: body.name,
            description: body.description,
            company_id: body.company_id,
            created_at: Utc::now(),
            updated_at: Some(Utc::now())
        };

        match self.repo.create(&role).await {
            Ok(_) => Ok("Role succesfuly created".to_string()),
            Err(e) => {
                return Err(ErrorInternalServerError(format!("Gagal membuat role: {}", e)));
            }
        }
    }

    pub async fn get_all(&self, query: PaginationQuery) -> Result<(Vec<RolesResponseDTO>, PaginationMeta), actix_web::Error> {
        let page = query.get_page();
        let limit = query.get_limit();
        let offset = query.get_offset();

        let (data_db, total_items) = self.repo.index(limit, offset)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        let data: Vec<RolesResponseDTO> = data_db
            .into_iter()
            .map(RolesResponseDTO::from)
            .collect();

        let meta = PaginationMeta::new(page, limit, total_items);

        return Ok((data, meta));
            
    }

}