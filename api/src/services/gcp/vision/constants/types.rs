use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectTextResponse {
    pub responses: Vec<TextResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextResponse {
    pub text_annotations: Option<Vec<TextAnnotation>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextAnnotation {
    pub description: String,
    pub bounding_poly: BoundingPoly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoundingPoly {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}
