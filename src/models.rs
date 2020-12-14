pub mod models {
    use serde::{
        Serialize,
        Deserialize
    };

    #[derive(Serialize, Deserialize)]
    pub struct Somestruct {
        pub rediskey: String,
    }
}
