use crate::entity::owner;
use crate::service::current_timestamp_millis;
use crate::{Error, Result};
use sea_orm::{DbConn, entity::*};
use uuid::{NoContext, Timestamp, Uuid};

pub struct OwnerService;

impl OwnerService {
    pub async fn create(db: &DbConn, name: String, email: Option<String>) -> Result<owner::Model> {
        let ts = Timestamp::now(NoContext);
        let now_millis = current_timestamp_millis();
        let owner = owner::ActiveModel {
            id: Set(Uuid::new_v7(ts).to_string()),
            name: Set(name),
            email: Set(email),
            created_at: Set(now_millis),
            updated_at: Set(now_millis),
            ..Default::default()
        };

        owner
            .insert(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }

    pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<owner::Model>> {
        owner::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }

    pub async fn update(
        db: &DbConn,
        id: String,
        name: Option<String>,
        email: Option<Option<String>>,
    ) -> Result<owner::Model> {
        let mut owner = Self::find_by_id(db, &id)
            .await?
            .ok_or_else(|| Error::NotFound("Owner not found".to_string()))?;

        if let Some(n) = name {
            owner.name = n;
        }
        if let Some(e) = email {
            owner.email = e;
        }
        owner.updated_at = current_timestamp_millis();

        let active = owner.into_active_model();

        active
            .update(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }

    pub async fn delete(db: &DbConn, id: &str) -> Result<()> {
        owner::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn list(db: &DbConn) -> Result<Vec<owner::Model>> {
        owner::Entity::find()
            .all(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}

#[cfg(test)]
mod test;
