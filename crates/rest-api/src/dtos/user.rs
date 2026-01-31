use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalUserDto {
    id: Uuid,
    name: String,
}

impl From<application::queries::get_groups_for_user::UserSummary> for MinimalUserDto {
    fn from(u: application::queries::get_groups_for_user::UserSummary) -> Self {
        Self {
            id: u.id.value(),
            name: u.name.value(),
        }
    }
}

impl From<application::models::user::UserSummary> for MinimalUserDto {
    fn from(u: application::models::user::UserSummary) -> Self {
        Self {
            id: u.id.value(),
            name: u.name.value(),
        }
    }
}
