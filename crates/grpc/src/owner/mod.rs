use crate::pb::{
    CreateOwnerRequest, GetOwnerRequest, OwnerResponse, owner_service_server::OwnerService,
};
use tonic::{Request, Response, Status};

pub struct OwnerGrpcService;

#[tonic::async_trait]
impl OwnerService for OwnerGrpcService {
    async fn create_owner(
        &self,
        request: Request<CreateOwnerRequest>,
    ) -> Result<Response<OwnerResponse>, Status> {
        let req = request.into_inner();
        let response = OwnerResponse {
            id: "placeholder-id".to_string(),
            name: req.name,
            email: req.email,
            phone: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            created_at: 0,
            updated_at: 0,
        };
        Ok(Response::new(response))
    }

    async fn get_owner(
        &self,
        request: Request<GetOwnerRequest>,
    ) -> Result<Response<OwnerResponse>, Status> {
        let _id = request.into_inner().id;
        let response = OwnerResponse {
            id: "placeholder-id".to_string(),
            name: "Placeholder Owner".to_string(),
            email: None,
            phone: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            created_at: 0,
            updated_at: 0,
        };
        Ok(Response::new(response))
    }
}
