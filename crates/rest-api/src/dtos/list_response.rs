use serde::Serialize;

use crate::dtos::pagination::PaginationDto;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T: Serialize> {
    pub data: Vec<T>,
    pub request_pagination: PaginationDto,
    pub total_items: usize,
}
