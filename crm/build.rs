use anyhow::Result;
use prost_build::Config;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let mut config = Config::new();
    config
        .out_dir("src/pb")
        .compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    Ok(())
}
