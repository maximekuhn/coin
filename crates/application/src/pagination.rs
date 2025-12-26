use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    page: NonZeroUsize,
    page_size: NonZeroUsize,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("page must not exceed {}", Pagination::MAX_PAGE)]
    PageLimitReached,

    #[error("page size must not exceed {}", Pagination::MAX_PAGE_SIZE)]
    PageSizeLimitReached,
}

impl Pagination {
    const MAX_PAGE: usize = 1000;
    const MAX_PAGE_SIZE: usize = 1000;

    const DEFAULT_PAGE: NonZeroUsize = NonZeroUsize::new(1).expect("1 > 0");
    const DEFAULT_PAGE_SIZE: NonZeroUsize = NonZeroUsize::new(10).expect("10 > 0");

    pub fn new(page: NonZeroUsize, page_size: NonZeroUsize) -> Result<Self, Error> {
        if page.get() > Self::MAX_PAGE {
            return Err(Error::PageLimitReached);
        }
        if page_size.get() > Self::MAX_PAGE_SIZE {
            return Err(Error::PageSizeLimitReached);
        }
        Ok(Self { page, page_size })
    }

    pub fn new_from_optional(
        page: Option<NonZeroUsize>,
        page_size: Option<NonZeroUsize>,
    ) -> Result<Self, Error> {
        let page = page.unwrap_or(Self::DEFAULT_PAGE);
        let page_size = page_size.unwrap_or(Self::DEFAULT_PAGE_SIZE);
        Self::new(page, page_size)
    }

    pub fn page(&self) -> NonZeroUsize {
        self.page
    }

    pub fn page_size(&self) -> NonZeroUsize {
        self.page_size
    }
}

// clippy: database cannot depend on application as this would create a circular
// dependency. That's why we need to convert from application to database.
#[allow(clippy::from_over_into)]
impl Into<database::DbPagination> for Pagination {
    fn into(self) -> database::DbPagination {
        database::DbPagination {
            limit: self.page_size.get(),
            offset: (self.page.get() - 1) * self.page_size.get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use crate::pagination::{Error, Pagination};

    #[rstest::rstest]
    #[case(1, 10, 10, 0)]
    #[case(2, 10, 10, 10)]
    #[case(2, 20, 20, 20)]
    fn db_conversion_is_correct(
        #[case] page: usize,
        #[case] page_size: usize,
        #[case] expected_limit: usize,
        #[case] expected_offset: usize,
    ) {
        let p = Pagination {
            page: NonZeroUsize::new(page).unwrap(),
            page_size: NonZeroUsize::new(page_size).unwrap(),
        };
        let dbp: database::DbPagination = p.into();
        assert_eq!(expected_limit, dbp.limit);
        assert_eq!(expected_offset, dbp.offset);
    }

    #[rstest::rstest]
    #[case(1, 10)]
    #[case(1, 999)]
    #[case(999, 345)]
    #[case(1_000, 1_000)]
    fn limits_ok(#[case] page: usize, #[case] page_size: usize) {
        assert!(
            Pagination::new(
                NonZeroUsize::new(page).unwrap(),
                NonZeroUsize::new(page_size).unwrap()
            )
            .is_ok()
        );
    }

    #[test]
    fn page_limit_err() {
        let page = NonZeroUsize::new(1_001).unwrap();
        let page_size = NonZeroUsize::new(10).unwrap();
        let err = Pagination::new(page, page_size).unwrap_err();
        assert_eq!(Error::PageLimitReached, err);
    }

    #[test]
    fn page_size_limit_err() {
        let page = NonZeroUsize::new(1).unwrap();
        let page_size = NonZeroUsize::new(1_001).unwrap();
        let err = Pagination::new(page, page_size).unwrap_err();
        assert_eq!(Error::PageSizeLimitReached, err);
    }

    #[test]
    fn from_optional_uses_default() {
        let pagination = Pagination::new_from_optional(None, None).unwrap();
        assert_eq!(1, pagination.page.get());
        assert_eq!(10, pagination.page_size.get());
    }

    #[test]
    fn from_optional_uses_provided_page() {
        let pagination =
            Pagination::new_from_optional(Some(NonZeroUsize::new(23).unwrap()), None).unwrap();
        assert_eq!(23, pagination.page.get());
        assert_eq!(10, pagination.page_size.get());
    }

    #[test]
    fn from_optional_uses_provided_page_size() {
        let pagination =
            Pagination::new_from_optional(None, Some(NonZeroUsize::new(87).unwrap())).unwrap();
        assert_eq!(1, pagination.page.get());
        assert_eq!(87, pagination.page_size.get());
    }
}
