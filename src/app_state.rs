use sqlx::MySqlPool;

//user
use crate::repository::user_repository::UserRepository;
use crate::services::users_service::UsersService;

//auth
use crate::services::auth_service::AuthService;


//roles 
use crate::repository::roles_repository::RolesRepository;
use crate::services::roles_service::RolesService;

pub struct AppState {
    pub auth_service: AuthService,
    pub users_service: UsersService,
    pub role_service : RolesService
}

impl AppState {
    pub fn new(pool : MySqlPool) -> Self {
        let user_repo = UserRepository::new(pool.clone());
        let role_repo = RolesRepository::new(pool.clone());

        let users_service = UsersService::new(user_repo.clone());
        let auth_service = AuthService::new(user_repo);
        let role_service = RolesService::new(role_repo);

        Self {
            auth_service,
            users_service,
            role_service,
        }
    }
}

