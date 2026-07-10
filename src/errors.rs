use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse{
    status: String,
    message: String
}

pub enum ErrorMessage {
    EmptyPassword,
    HashingError,
    Unauthorized,
    InvalidCredentials,
    TokenFailed,
    UserNotExist,
    EmailError,
    CreateUserFailed,
    UserAlreadyExist,
    InvalidInput,
    CreateRoomFailed,
    FetchAllUsersFailed,
    FailedMessage,
    SocketFailed,
    AddMemberFailed,
    AddMemberFailedV2,
    RoomNotFound,
    FailedToGetMsg,
    RoomAlreadyExist,
    MemberNotExist
}

impl ErrorMessage {
    pub fn return_err(&self) -> String{
        match self {
            ErrorMessage::EmptyPassword => "Password Input is empty!!".to_string(),
            ErrorMessage::HashingError => "Password Hashing Failed!!".to_string(),
            ErrorMessage::Unauthorized => "Function failed: Unauthorized".to_string(),
            ErrorMessage::InvalidCredentials => "Function Failed: InvalidCredentials".to_string(),
            ErrorMessage::TokenFailed => "Failed to initialize token".to_string(),
            ErrorMessage::UserNotExist => "Server Failed, User does not exist".to_string(),
            ErrorMessage::CreateUserFailed => "Failed to create user".to_string(),
            ErrorMessage::UserAlreadyExist => "Failed to create user, User already exists!!".to_string(),
            ErrorMessage::InvalidInput => "Invalid Input!!".to_string(),
            ErrorMessage::CreateRoomFailed => "Failed to create room".to_string(),
            ErrorMessage::FetchAllUsersFailed => "Failed to fetch all users".to_string(),
            ErrorMessage::FailedMessage => "Failed to send message".to_string(),
            ErrorMessage::SocketFailed => "Failed to create websocket connection".to_string(),
            ErrorMessage::AddMemberFailed => "Failed to add room member".to_string(),
            ErrorMessage::AddMemberFailedV2 => "Failed to add room member, they dont exist".to_string(),
            ErrorMessage::RoomNotFound => "Failed to find room, it does not exist".to_string(),
            ErrorMessage::FailedToGetMsg => "Failed to get messages".to_string(),
            ErrorMessage::RoomAlreadyExist => "Chat room already exists".to_string(),
            ErrorMessage::MemberNotExist => "room member doesnt exist!!".to_string(),
            ErrorMessage::EmailError => "Failed to send email!!".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError{
    message: String,
    status: StatusCode
}

impl HttpError {
    pub fn bad_service(message: impl Into<String>) -> Self{
        HttpError { message: message.into(), status: StatusCode::BAD_REQUEST }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self{
        HttpError { message: message.into(), status: StatusCode::UNAUTHORIZED }
    }
    
    pub fn server_error(message: impl Into<String>) -> Self{
        HttpError { message: message.into(), status: StatusCode::INTERNAL_SERVER_ERROR }
    }

    pub fn into_http_response(self) -> Response{
        let json_response = Json(ErrorResponse{
            message: self.message.clone(),
            status: "fail".to_string()
        });

        (self.status, json_response).into_response()
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}