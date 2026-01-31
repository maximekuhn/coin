use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationDto {
    page: usize,
    page_size: usize,
}

impl From<application::pagination::Pagination> for PaginationDto {
    fn from(p: application::pagination::Pagination) -> Self {
        Self {
            page: p.page().get(),
            page_size: p.page_size().get(),
        }
    }
}
