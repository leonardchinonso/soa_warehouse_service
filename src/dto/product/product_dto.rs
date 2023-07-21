use serde::Deserialize;
// AddProductRequest represents the request body for adding a product
#[derive(Deserialize)]
pub struct AddProductRequest {
    pub name: String,
    pub description: String,
}

