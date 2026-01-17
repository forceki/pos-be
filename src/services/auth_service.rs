use crate::dtos::auth_dto::{LoginUserDTO, RegisterUserDTO, UserResponseDto, LoginResponseDto};
use crate::repository::user_repository::UserRepository;
use crate::models::user_model::User;
use crate::utils::token_utils;
use uuid::Uuid;
use chrono::Utc;
use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordVerifier},
    Argon2
};

use actix_web::error::ErrorInternalServerError;

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(repo: UserRepository) -> Self {
        AuthService { repo }
    }

    pub async fn register_user(&self, dto: RegisterUserDTO) -> Result<String, actix_web::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| ErrorInternalServerError("Gagal hash password"))?
            .to_string();

        let new_user = User {
            user_id: Uuid::new_v4().to_string(),
            username: dto.username,
            email: dto.email,
            fullname: dto.fullname,
            password: password_hash,
            role_id: dto.role_id,
            tenant_id: dto.tenant_id,
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

    pub async fn login_user(&self, dto: LoginUserDTO) -> Result<LoginResponseDto, actix_web::Error> {
        let user_option = self.repo.find_by_email(&dto.email).await
            .map_err(|e| ErrorInternalServerError(format!("Gagal mencari user: {}", e)))?;

        let user = match user_option {
            Some(u) => u,
            None => return Err(ErrorInternalServerError("Email tidak ditemukan")),
        };

        let parsed_hash = argon2::PasswordHash::new(&user.password)
            .map_err(|_| ErrorInternalServerError("Gagal memverifikasi password"))?;

        let argon2 = Argon2::default();

        if argon2.verify_password(dto.password.as_bytes(), &parsed_hash).is_err() {
            return Err(ErrorInternalServerError("Email atau Password salah"));
        }

        let token = token_utils::generate_token(&user.user_id.to_string(), &user.role_id.to_string()) // Sesuaikan user_id u64/string
            .map_err(|e| ErrorInternalServerError(e.to_string()))?;

        let login_response = LoginResponseDto {
            token,
            user: UserResponseDto::from(user),
        };

        return Ok(login_response);

    }
}