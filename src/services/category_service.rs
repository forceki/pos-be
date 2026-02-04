
use crate::models::category_model::Category;
use crate::repository::category_repository::CategoryRepository;
use crate::dtos::category_dto::{CategoryQuery, CategoryResponse, CreateCategoryDto, UpdateCategoryDto};
use crate::utils::pagination::PaginationMeta;
use slug::slugify; 
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};

pub struct CategoryService {
    repo: CategoryRepository,
}

impl CategoryService {
    pub fn new(repo: CategoryRepository) -> Self {
        Self { repo }
    }

    pub async fn get_tree(&self, query: CategoryQuery) -> Result<Vec<CategoryResponse>, actix_web::Error> {
        let limit = query.pagination.get_limit();
        let offset = query.pagination.get_offset();
        let search = query.search;

        let (flat_list, _) = self.repo.get_all(limit, offset, search).await
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(Self::build_tree(flat_list, None))
    }

    fn build_tree(categories: Vec<Category>, parent_id: Option<u64>) -> Vec<CategoryResponse> {
        categories
            .iter()
            .filter(|c| c.parent_id == parent_id)
            .map(|c| CategoryResponse {
                id: c.id,
                name: c.name.clone(),
                slug: c.slug.clone(),
                parent_id: c.parent_id,
                is_active: c.is_active,
                children: Self::build_tree(categories.clone(), Some(c.id)),
            })
            .collect()
    }

    pub async fn get_all(&self, query: CategoryQuery) -> Result<(Vec<CategoryResponse>, PaginationMeta), actix_web::Error> {
        let page = query.pagination.get_page();
        let limit = query.pagination.get_limit();
        let offset = query.pagination.get_offset();
        let search = query.search;

        let (data_db, total_items) = self.repo.get_all(limit, offset, search)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

       let data_dto: Vec<CategoryResponse> = data_db
            .into_iter()
            .map(CategoryResponse::from)
            .collect();

        let meta = PaginationMeta::new(page, limit, total_items);

        return Ok((data_dto, meta));
    }

    pub async fn create(&self, dto: CreateCategoryDto, user_id: &str) -> Result<u64, actix_web::Error> {
        let slug = slugify(&dto.name);
        
        self.repo.create(&dto.name, &slug, dto.parent_id, user_id)
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }

    pub async fn update(&self, id: u64, dto: UpdateCategoryDto, user_id: &str) -> Result<(), actix_web::Error> {
        let affected = self.repo.update(id, dto, user_id)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        if affected == 0 {
            return Err(ErrorNotFound("Category not found"));
        }
        Ok(())
    }
}