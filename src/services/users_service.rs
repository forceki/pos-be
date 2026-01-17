
use crate::dtos::auth_dto::{UserResponseDto};
use crate::dtos::pagination_dto::{PaginationQuery};
use crate::repository::user_repository::UserRepository;
use crate::utils::pagination::{PaginationMeta};

use actix_web::error::ErrorInternalServerError;

pub struct UsersService{
    repo: UserRepository
}

impl UsersService {
    pub fn new(repo: UserRepository) -> Self {
        UsersService { repo }
    }

    pub async fn get_all_users(&self, query: PaginationQuery) -> Result<(Vec<UserResponseDto>, PaginationMeta), actix_web::Error> {
        let page = query.get_page();
        let limit = query.get_limit();
        let offset = query.get_offset();

        let (users_db, total_items) = self.repo.fetch_all(limit, offset)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        let users_dto: Vec<UserResponseDto> = users_db
            .into_iter()
            .map(UserResponseDto::from)
            .collect();

        let meta = PaginationMeta::new(page, limit, total_items);

        return Ok((users_dto, meta));
    }

    pub async fn user_by_id(&self, user_id: &str) -> Result<Option<UserResponseDto>, actix_web::Error> {
        
        let user = self.repo.find_by_id(user_id).await
                .map_err(|e| ErrorInternalServerError(format!("Gagal mencari user: {}", e)))?
                .map(UserResponseDto::from);

        return Ok(user);
    }
}