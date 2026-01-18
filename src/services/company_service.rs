use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use uuid::Uuid;

use crate::{dtos::company_dto::CreateCompanyDTO, models::company_model::Company, repository::company_repository::CompanyRepository, utils::text_utils};




pub struct CompanyService {
    repo: CompanyRepository
}

impl CompanyService {
    pub fn new(repo: CompanyRepository) -> Self {
        CompanyService { repo }
    }

    pub async fn create(&self, body: CreateCompanyDTO) -> Result<String, actix_web::Error> {
        let raw_slug = text_utils::to_slug(&body.name);
        let payload = Company {
            id: Uuid::new_v4().to_string(),
            name: body.name,
            slug: raw_slug,
            address: body.address,
            phone_number: body.phone_number,
            link: body.link,
            created_at: Utc::now(),
            updated_at: Utc::now()
        };

        match self.repo.create(&payload).await {
            Ok(_) => Ok("Role succesfuly created".to_string()),
            Err(e) => {
                return Err(ErrorInternalServerError(format!("Gagal membuat role: {}", e)));
            }
        }
    }
}

