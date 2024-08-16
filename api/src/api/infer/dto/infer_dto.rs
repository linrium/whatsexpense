use crate::api::category::Category;

#[derive(Clone, Debug)]
pub struct InferOptions {
    pub currencies: Vec<String>,
    pub categories: Vec<Category>,
}
