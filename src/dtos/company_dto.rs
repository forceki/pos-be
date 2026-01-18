use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

use crate::models::company_model::Company;


#[derive(Deserialize)]
pub struct CreateCompanyDTO{
    pub name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub link : Option<String>,

}

#[derive(Serialize)]
pub struct CompanyResponseDTO{
    pub id: String,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub link : Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl From<Company> for CompanyResponseDTO {
    fn from(company: Company) -> Self{
        CompanyResponseDTO {
            id: company.id,
            name: company.name,
            slug: company.slug,
            address: company.address,
            phone_number: company.phone_number,
            link: company.link,
            created_at: company.created_at,
            updated_at: company.updated_at
        }
    }
}