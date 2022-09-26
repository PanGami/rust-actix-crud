#[derive(serde::Serialize, Clone)]
pub enum AuthenticationSource {
    Google = 1,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct User {
    id: Option<u64>,
    display_name: String,
    source: AuthenticationSource,
    external_id: String,
}

impl User {
    pub fn new(display_name: String, source: AuthenticationSource, external_id: String) -> User {
        User {
            display_name,
            source,
            external_id,
            id: None,
        }
    }

    pub fn exists(self) -> bool {
        self.id.is_some()
    }
}