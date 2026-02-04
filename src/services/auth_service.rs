use crate::dtos::auth_dto::{CreateOnboardDTO, LoginResponseDto, LoginUserDTO, OnboardUserDTO, RegisterUserDTO, UserResponseDto};
use crate::repository::auth_repository::AuthRepository;
use crate::utils::{text_utils, token_utils};
use uuid::Uuid;
use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordVerifier},
    Argon2
};

use actix_web::error::ErrorInternalServerError;

pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(repo: AuthRepository) -> Self {
        AuthService { repo }
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
    
    pub async fn onboard(&self, dto: OnboardUserDTO) -> Result<String, actix_web::Error> {
        
        let hashed_password = Self::generate_password(&dto.password)?;
        let raw_slug = text_utils::to_slug(&dto.company_name);

        let payload = CreateOnboardDTO{
            user_id: Uuid::new_v4().to_string(),
            fullname: dto.fullname,
            email: dto.email,
            password: hashed_password,
            company_id : Uuid::new_v4().to_string(),
            company_name: dto.company_name,
            company_slug: raw_slug,
            role_id: Uuid::new_v4().to_string(),
            role_name: "Owner".to_string()
        };

        let user_id = self.repo.onboard(payload)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(user_id)
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

        let token = token_utils::generate_token(&user.user_id.to_string(), &user.role_id.to_string(), &user.company_id.to_string()) // Sesuaikan user_id u64/string
            .map_err(|e| ErrorInternalServerError(e.to_string()))?;

        let login_response = LoginResponseDto {
            token,
            user: UserResponseDto::from(user),
        };

        return Ok(login_response);

    }
}