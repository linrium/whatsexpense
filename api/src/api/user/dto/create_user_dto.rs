pub struct CreateUserData {
    pub full_name: String,
    pub email: String,
    pub encrypted_password: String,
    pub family_name: String,
    pub given_name: String,
    pub picture: String,
    pub username: String,
    pub currency: String,
    pub language: String,
    pub regions: Vec<String>,
}

pub struct CreateUserInput {
    pub full_name: String,
    pub email: String,
    pub encrypted_password: String,
    pub family_name: String,
    pub given_name: String,
    pub picture: String,
    pub username: String,
    pub currency: String,
    pub language: String,
    pub regions: Vec<String>,
}

impl From<CreateUserInput> for CreateUserData {
    fn from(input: CreateUserInput) -> Self {
        Self {
            full_name: input.full_name,
            email: input.email,
            encrypted_password: input.encrypted_password,
            family_name: input.family_name,
            given_name: input.given_name,
            picture: input.picture,
            username: input.username,
            currency: input.currency,
            language: input.language,
            regions: input.regions,
        }
    }
}
