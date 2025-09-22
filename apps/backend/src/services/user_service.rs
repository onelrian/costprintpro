use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    db::Database,
    models::{CreateUserRequest, UpdateUserRequest, User, UserRole},
    utils::errors::AppError,
};

pub struct UserService<'a> {
    db: &'a sea_orm::DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

        let user = User {
            id: Uuid::new_v4(),
            email: request.email,
            password_hash,
            role: request.role.unwrap_or(UserRole::User),
            first_name: request.first_name,
            last_name: request.last_name,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // In a real implementation, this would use SeaORM to insert into database
        // For now, we'll return the user (this is a placeholder)
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // Placeholder implementation - in reality this would query the database
        // For demo purposes, create a default admin user
        if email == "admin@costprint.com" {
            let password_hash = hash("admin123", DEFAULT_COST)
                .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

            Ok(Some(User {
                id: Uuid::new_v4(),
                email: email.to_string(),
                password_hash,
                role: UserRole::Admin,
                first_name: Some("Admin".to_string()),
                last_name: Some("User".to_string()),
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        // Placeholder implementation
        Ok(None)
    }

    pub async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<User, AppError> {
        // Placeholder implementation
        Err(AppError::NotFound("User not found".to_string()))
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
        // Placeholder implementation
        Ok(())
    }

    pub async fn list_users(&self, page: u64, limit: u64) -> Result<Vec<User>, AppError> {
        // Placeholder implementation
        Ok(vec![])
    }
}
