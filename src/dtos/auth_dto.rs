use serde::{Deserialize, Serialize};

use crate::models::user_model::User;

#[derive(Deserialize)]
pub struct RegisterUserDTO {
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub role_id: String,
    pub tenant_id: String,
}

#[derive(Deserialize)]
pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct LoginResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}


#[derive(Serialize)]
pub struct UserResponseDto {
    pub user_id: String,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub role_id: String,
    pub tenant_id: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        UserResponseDto {
            user_id: user.user_id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            role_id: user.role_id,
            tenant_id: user.tenant_id,
        }
    }
}