use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub sub: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

impl UserProfile {
    pub fn full_name(&self) -> String {
        let family_name = self.family_name.clone().unwrap_or_default();
        let given_name = self.given_name.clone().unwrap_or_default();
        format!("{} {}", given_name, family_name)
    }

    pub fn generate_username(&self) -> String {
        // Convert the name to lowercase and replace spaces with dots
        let mut username = self.full_name().to_lowercase().replace(" ", ".");

        // Generate a random string of length 5 using nanoid
        let random_string = nanoid!(5);

        // Append the random string to the username
        username.push('_');
        username.push_str(&random_string);

        username
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub role: Role,
}

// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
// where
//     S: Send + Sync,
// {
//     type Rejection = AuthError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken)?;
//
//         let token_data = decode::<Claims>(bearer.token(), &"".to_string(), &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//
//         Ok(token_data.claims)
//     }
// }
