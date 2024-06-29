use anyhow::Result;
use crm::pb::User;
use prost::Message;

fn main() -> Result<()> {
    let user = User::new(1, "John Doe", "john.doe@example.com");
    println!("user: {:?}", user);
    let encoded = user.encode_to_vec();
    println!("encoded: {:?}", encoded);
    let decoded = User::decode(&encoded[..])?;
    println!("decoded: {:?}", decoded);
    Ok(())
}
