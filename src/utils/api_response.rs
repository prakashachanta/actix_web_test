use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String
}

// pub struct CreateUserResponse{
//     pub id: u32,
//     pub name: String,
//     pub email: String
// }

// #[derive(Deserialize)]
// pub struct CreateUserData {
//     pub name: String,
//     pub email: String
// }