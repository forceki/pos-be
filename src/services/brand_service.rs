use crate::models::brand_model::Brand;
use crate::repository::brand_repository::BrandRepository;
use crate::dtos::brand_dto::{CreateBrandDto, UpdateBrandDto};
use crate::utils::pagination::PaginationMeta;
use slug::slugify;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};

pub struct BrandService {
    repo: BrandRepository,
}

impl BrandService {
    pub fn new(repo: BrandRepository) -> Self {
        Self { repo }
    }

    pub async fn create_brand(&self, dto: CreateBrandDto, user_id: &str) -> Result<u64, actix_web::Error> {
        let slug = slugify(&dto.name);
        self.repo.create(&dto.name, &slug, user_id)
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }

    pub async fn get_brands(&self, limit: i64, offset: i64, search: Option<String>, page: i64) -> Result<(Vec<Brand>, PaginationMeta), actix_web::Error> {
        let (brands, total_items) = self.repo.get_all(limit, offset, search)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        let meta = PaginationMeta {
            page,
            limit,
            total_items,
            total_pages: (total_items as f64 / limit as f64).ceil() as i64,
        };

        Ok((brands, meta))
    }

    pub async fn update_brand(&self, id: u64, dto: UpdateBrandDto, user_id: &str) -> Result<(), actix_web::Error> {
        let affected = self.repo.update(id, dto, user_id)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        if affected == 0 {
            return Err(ErrorNotFound("Brand not found"));
        }
        Ok(())
    }
}