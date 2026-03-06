use crate::pb::{
    CreateOwnerRequest, DeleteOwnerRequest, DeleteOwnerResponse, GetOwnerRequest,
    ListOwnersRequest, ListOwnersResponse, OwnerResponse, UpdateOwnerRequest,
    owner_service_server::OwnerService,
};
use ee_vpms_core::service::OwnerService as CoreOwnerService;
use ee_vpms_core::entity::owner;
use sea_orm::DbConn;
use tonic::{Request, Response, Status};

pub struct OwnerGrpcService {
    pub db: DbConn,
}

fn to_owner_response(model: owner::Model) -> OwnerResponse {
    OwnerResponse {
        id: model.id,
        name: model.name,
        email: model.email,
        phone: model.phone,
        address: model.address,
        city: model.city,
        state: model.state,
        postal_code: model.postal_code,
        country: model.country,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

#[tonic::async_trait]
impl OwnerService for OwnerGrpcService {
    async fn create_owner(
        &self,
        request: Request<CreateOwnerRequest>,
    ) -> Result<Response<OwnerResponse>, Status> {
        let req = request.into_inner();
        match CoreOwnerService::create(&self.db, req.name, req.email).await {
            Ok(model) => Ok(Response::new(to_owner_response(model))),
            Err(_) => Err(Status::internal("Failed to create owner")),
        }
    }

    async fn get_owner(
        &self,
        request: Request<GetOwnerRequest>,
    ) -> Result<Response<OwnerResponse>, Status> {
        let req = request.into_inner();
        match CoreOwnerService::find_by_id(&self.db, &req.id).await {
            Ok(Some(model)) => Ok(Response::new(to_owner_response(model))),
            Ok(None) => Err(Status::not_found("Owner not found")),
            Err(_) => Err(Status::internal("Database error")),
        }
    }

    async fn list_owners(
        &self,
        _request: Request<ListOwnersRequest>,
    ) -> Result<Response<ListOwnersResponse>, Status> {
        match CoreOwnerService::list(&self.db).await {
            Ok(models) => {
                let owners = models.into_iter().map(to_owner_response).collect();
                Ok(Response::new(ListOwnersResponse { owners }))
            }
            Err(_) => Err(Status::internal("Database error")),
        }
    }

    async fn update_owner(
        &self,
        request: Request<UpdateOwnerRequest>,
    ) -> Result<Response<OwnerResponse>, Status> {
        let req = request.into_inner();
        let email = if req.email.is_some() {
            Some(req.email)
        } else {
            None
        };
        match CoreOwnerService::update(&self.db, req.id, req.name, email).await {
            Ok(model) => Ok(Response::new(to_owner_response(model))),
            Err(_) => Err(Status::internal("Failed to update owner")),
        }
    }

    async fn delete_owner(
        &self,
        request: Request<DeleteOwnerRequest>,
    ) -> Result<Response<DeleteOwnerResponse>, Status> {
        let req = request.into_inner();
        match CoreOwnerService::delete(&self.db, &req.id).await {
            Ok(()) => Ok(Response::new(DeleteOwnerResponse { success: true })),
            Err(_) => Err(Status::internal("Failed to delete owner")),
        }
    }
}
