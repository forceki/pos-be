use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::models::roles_model::Roles;

#[derive(Deserialize)]
pub struct CreateRolesDTO{
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String
}


#[derive(Serialize)]
pub struct RolesResponseDTO{
    pub id: String, 
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub created_at: DateTime<Utc>
}

impl From<Roles> for RolesResponseDTO {
    fn from(role: Roles) -> Self {
        RolesResponseDTO { 
            id: role.id, 
            name: role.name, 
            description: role.description, 
            tenant_id: role.tenant_id,
            created_at: role.created_at
        }
    }
}