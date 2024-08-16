#[macro_export]
macro_rules! object_id {
    ($id:expr) => {{
        use std::str::FromStr;
        bson::oid::ObjectId::from_str($id).unwrap()
    }};
}

pub use object_id;
