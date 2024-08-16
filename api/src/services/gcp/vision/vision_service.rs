use crate::services::gcp::auth::GCPAuthServiceDyn;
use crate::services::gcp::vision::constants::GCPVisionError;
use crate::services::gcp::vision::types::DetectTextResponse;
use anyhow::anyhow;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;

#[async_trait]
pub trait VisionServiceExt {
    async fn detect_text(&self, url: String) -> Result<String, GCPVisionError>;
}

pub type VisionServiceDyn = Arc<dyn VisionServiceExt + Send + Sync>;

pub struct VisionService {
    pub http_client: reqwest::Client,
    pub gcp_auth_service: GCPAuthServiceDyn,
}

#[async_trait]
impl VisionServiceExt for VisionService {
    async fn detect_text(&self, content: String) -> Result<String, GCPVisionError> {
        let token = self
            .gcp_auth_service
            .get_access_token()
            .await
            .map_err(|e| GCPVisionError::Unknown(e.into()))?;

        let response = self
            .http_client
            .post("https://vision.googleapis.com/v1/images:annotate")
            .bearer_auth(token)
            .json(&json!({
              "requests": [
                {
                  "image": {
                    "content": content
                  },
                  "features": [
                    {
                      "type": "TEXT_DETECTION"
                    }
                  ]
                }
              ]
            }))
            .send()
            .await
            .map_err(|e| GCPVisionError::Unknown(e.into()))?
            .json::<DetectTextResponse>()
            .await
            .map_err(|e| GCPVisionError::Unknown(anyhow!("failed to deserialize response: {e}")))?;

        let mut annotations = response
            .responses
            .first()
            .ok_or(GCPVisionError::Unknown(anyhow!("no response")))?
            .text_annotations
            .clone()
            .unwrap_or_default();

        let mut contents: Vec<Vec<(String, i32, i32)>> = vec![vec![]];
        let mut current_line_y = 0;
        let mut current_row = 0;

        // Sort annotations by y
        annotations.sort_by(|a, b| {
            a.bounding_poly.vertices[0]
                .y
                .cmp(&b.bounding_poly.vertices[0].y)
        });

        for annotation in annotations {
            if current_row == 0 {
                current_row += 1;
                continue;
            }

            let vertices = &annotation.bounding_poly.vertices;
            let text = &annotation.description;
            let line_y = vertices[0].y;
            let line_x = vertices[0].x;
            if current_line_y == 0 {
                current_line_y = line_y;
            }

            if line_y <= current_line_y + 10 {
                contents
                    .last_mut()
                    .unwrap()
                    .push((text.clone(), line_x, line_y));
            } else {
                contents.push(vec![(text.clone(), line_x, line_y)]);
                current_line_y = line_y;
                current_row += 1;
            }
        }

        // Sort contents by x
        for content in &mut contents {
            content.sort_by(|a, b| a.1.cmp(&b.1));
        }

        let description = contents
            .into_iter()
            .map(|content| {
                content
                    .into_iter()
                    .map(|line| line.0)
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join(", ");
        debug!("description: {description}");

        Ok(description)
    }
}
