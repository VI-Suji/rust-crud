use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use crate::model::{User, UserInfo};

pub async fn list_users() -> (StatusCode, Json<Value>) {

}

pub async fn get_user_by_id(Path: Path<u64>) -> (StatusCode, Json<Value>) {

}

pub async fn create_user(Json: Json<UserInfo>) -> StatusCode {

}

pub async fn update_user(Path: Path<u64>, Json: Json<UserInfo>) -> StatusCode {

}

pub async fn delete_user(Path: Path<u64>) -> StatusCode {

}