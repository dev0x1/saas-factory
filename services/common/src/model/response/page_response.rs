use crate::model::domain::pagination::Pagination;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug, Clone)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub page_info: Pagination,
}

impl<T> fmt::Display for PageResponse<T>
where
    T: Serialize + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let processed = self
            .data
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",");
        write!(
            f,
            "{{ \"data\" : [{}], \"page_info\" : {} }}",
            processed,
            serde_json::to_string(&self.page_info).unwrap()
        )
    }
}
