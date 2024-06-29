use crate::pb::*;
use anyhow::Result;
use prost::Message;

pub mod pb {
    use prost_types::Timestamp;
    use std::time::SystemTime;

    include!(concat!(env!("OUT_DIR"), "/crm.rs"));

    impl User {
        pub fn new(id: u64, name: &str, email: &str) -> Self {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            Self {
                id,
                name: name.to_string(),
                email: email.to_string(),
                created_at: Some(Timestamp {
                    seconds: now.as_secs() as i64,
                    nanos: now.subsec_nanos() as i32,
                }),
            }
        }
    }
}

fn main() -> Result<()> {
    let user = User::new(1, "John Doe", "john.doe@example.com");
    println!("user: {:?}", user);
    let encoded = user.encode_to_vec();
    println!("encoded: {:?}", encoded);
    let decoded = User::decode(&encoded[..])?;
    println!("decoded: {:?}", decoded);
    Ok(())
}
