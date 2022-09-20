use actix_web::web::{Json};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::controllers::constants::APPLICATION_JSON;
use crate::controllers::response::Response;

use crate::factories::user_factory::UserFactory;
use crate::services::user_service::{UserService, Service};

pub type UserControllers = Response<UserController>;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserController {
    pub name: String,
}

impl UserController {
    pub fn new(name: String) -> Self {

        Self {
            name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub name: Option<String>,
}

impl UserRequest {
    pub fn to_user(&self) -> Option<UserController> {
        match &self.name {
            Some(name) => Some(UserController::new(name.to_string())),
            None => None,
        }
    }
}


#[post("/users")]
pub async fn create(user_req: Json<UserRequest>) -> HttpResponse {

    let result = user_req.name.as_ref();

    let mut user_factory = UserFactory::new();

    let entity = user_factory.create(&result.unwrap());
    
    let mut user_service = UserService::new();

    if let Err(err) = user_service.add(&entity) {
        HttpResponse::BadRequest().json(err)
    } else {
        HttpResponse::Created()
                            .content_type(APPLICATION_JSON)
                            .json(user_req.to_user())
    }
}
