use protox::prost::Message;
use std::{env, fs, path::PathBuf};

const PROTO_FILES: &[&str; 1] = &["occurrence/v1/occurrences_service.proto"];

//INFO: change this to update version
const BUF_SCHEMA_ZIP: &str =
    "https://buf.build/zizico2/prociv-reverse-proxy/archive/fb050446798842778343e11cf02dbfbe.zip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=migrations");
    let bytes = reqwest::get(BUF_SCHEMA_ZIP).await?.bytes().await?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let protos_dir = out_dir.join("protos");

    zip_extract::extract(std::io::Cursor::new(bytes), &protos_dir, true)?;

    let proto_includes: [&str; 1] = [protos_dir
        .to_str()
        .ok_or(anyhow::anyhow!("invalid protos_dir"))?];

    let file_descriptors = protox::compile(PROTO_FILES, proto_includes)?;
    let file_descriptor_path = out_dir.join("file_descriptor_set.bin");
    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec())?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .file_descriptor_set_path(&file_descriptor_path)
        .skip_protoc_run()
        .compile_protos(PROTO_FILES, &proto_includes)?;

    Ok(())
}
