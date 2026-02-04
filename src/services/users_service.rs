
use crate::dtos::auth_dto::{RegisterUserDTO, UserResponseDto};
use crate::dtos::pagination_dto::{PaginationQuery};
use crate::models::user_model::User;
use crate::repository::user_repository::UserRepository;
use crate::utils::pagination::{PaginationMeta};

use actix_web::error::ErrorInternalServerError;
use argon2::{
    password_hash::{SaltString, PasswordHasher},
    Argon2
};
use chrono::Utc;
use uuid::Uuid;

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

    fn generate_password(password: &str) -> Result<String, actix_web::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| ErrorInternalServerError(format!("Gagal hash: {}", e)))?
            .to_string();

        Ok(password_hash)
    }


    pub async fn register_user(&self, dto: RegisterUserDTO) -> Result<String, actix_web::Error> {
        // let salt = SaltString::generate(&mut rand::thread_rng());
        // let argon2 = Argon2::default();
        // let password_hash = argon2.hash_password(dto.password.as_bytes(), &salt)
        //     .map_err(|_| ErrorInternalServerError("Gagal hash password"))?
        //     .to_string();

        let hashed_password = Self::generate_password(&dto.password)?;

        let new_user = User {
            user_id: Uuid::new_v4().to_string(),
            email: dto.email,
            fullname: dto.fullname,
            password: hashed_password,
            role_id: dto.role_id,
            company_id: dto.company_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };


        match self.repo.create(&new_user).await {
            Ok(_) => Ok("User berhasil dibuat".to_string()),
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23000") => {
                return Err(ErrorInternalServerError("Email sudah terdaftar"));
            },
            Err(e) => {
                return Err(ErrorInternalServerError(format!("Gagal membuat user: {}", e)));
            }
        }
    }

    pub async fn user_by_id(&self, user_id: &str) -> Result<Option<UserResponseDto>, actix_web::Error> {
        
        let user = self.repo.find_by_id(user_id).await
                .map_err(|e| ErrorInternalServerError(format!("Gagal mencari user: {}", e)))?
                .map(UserResponseDto::from);

        return Ok(user);
    }
}