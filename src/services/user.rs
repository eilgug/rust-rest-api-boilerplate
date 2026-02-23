use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::profile;

pub async fn find_by_auth_id(
    db: &DatabaseConnection,
    auth_id: &str,
) -> Result<Option<profile::Model>, AppError> {
    Ok(profile::Entity::find()
        .filter(profile::Column::AuthId.eq(auth_id))
        .one(db)
        .await?)
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<profile::Model>, AppError> {
    Ok(profile::Entity::find_by_id(id).one(db).await?)
}

pub async fn create_profile(
    db: &DatabaseConnection,
    auth_id: String,
    email: String,
) -> Result<profile::Model, AppError> {
    let now = chrono::Utc::now().fixed_offset();

    let new_profile = profile::ActiveModel {
        id: Set(Uuid::new_v4()),
        auth_id: Set(auth_id),
        email: Set(email),
        display_name: Set(None),
        avatar_url: Set(None),
        bio: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    Ok(new_profile.insert(db).await?)
}

pub async fn update_profile(
    db: &DatabaseConnection,
    auth_id: &str,
    display_name: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
) -> Result<profile::Model, AppError> {
    let profile = find_by_auth_id(db, auth_id)
        .await?
        .ok_or_else(|| AppError::NotFound("profile not found".into()))?;

    let mut active: profile::ActiveModel = profile.into();

    if let Some(name) = display_name {
        active.display_name = Set(Some(name));
    }
    if let Some(text) = bio {
        active.bio = Set(Some(text));
    }
    if let Some(url) = avatar_url {
        active.avatar_url = Set(Some(url));
    }
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    Ok(active.update(db).await?)
}

pub async fn delete_profile(db: &DatabaseConnection, auth_id: &str) -> Result<(), AppError> {
    let profile = find_by_auth_id(db, auth_id)
        .await?
        .ok_or_else(|| AppError::NotFound("profile not found".into()))?;

    profile::Entity::delete_by_id(profile.id)
        .exec(db)
        .await?;

    Ok(())
}
