use sqlx::MySqlPool;

use crate::repository::auth_repository::AuthRepository;
//user
use crate::repository::user_repository::UserRepository;
use crate::services::users_service::UsersService;

//auth
use crate::services::auth_service::AuthService;

//roles 
use crate::repository::roles_repository::RolesRepository;
use crate::services::roles_service::RolesService;

//company 
use crate::repository::company_repository::CompanyRepository;
use crate::services::company_service::CompanyService;

pub struct AppState {
    pub db: MySqlPool, 
}

impl AppState {

    pub fn new(pool: MySqlPool) -> Self {
        Self { db: pool }
    }

    pub fn auth_service(&self) -> AuthService {
        let repo = AuthRepository::new(self.db.clone());
        AuthService::new(repo)
    }

    pub fn users_service(&self, tenant_id: String) -> UsersService {
        let repo = UserRepository::new(self.db.clone(), tenant_id);
        UsersService::new(repo)
    }

    pub fn roles_service(&self, tenant_id: String) -> RolesService {
        let repo = RolesRepository::new(self.db.clone(), tenant_id);
        RolesService::new(repo)
    }

    pub fn company_service(&self, tenant_id: String) -> CompanyService {
        let repo = CompanyRepository::new(self.db.clone(), tenant_id);
        CompanyService::new(repo)
    }

    // pub fn new(pool : MySqlPool) -> Self {
    //     let user_repo = UserRepository::new(pool.clone());
    //     let role_repo = RolesRepository::new(pool.clone());
    //     let company_repo = CompanyRepository::new(pool.clone());

    //     let users_service = UsersService::new(user_repo.clone());
    //     let auth_service = AuthService::new(user_repo);
    //     let role_service = RolesService::new(role_repo);
    //     let company_service = CompanyService::new(company_repo);

    //     Self {
    //         auth_service,
    //         users_service,
    //         role_service,
    //         company_service
    //     }
    // }
}

