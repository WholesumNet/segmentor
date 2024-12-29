use risc0_zkvm::{
    ApiClient,
    ExecutorEnv,
    Asset, AssetRequest,
};
 
use anyhow;

use clap::{
    Parser
};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    /// Path to elf file
    #[arg(short, long)]
    elf: String,

    /// Segment limit size
    #[arg(short, long, default_value = "20")]
    po2: String,

    /// Path to write segment blobs to
    #[arg(short, long)]
    out_path: String,    
}


fn main() -> anyhow::Result<()> {
    let args = Cli::parse();    
    let po2 = u32::from_str_radix(&args.po2, 10)?;

    let r0_client = ApiClient::from_env()?;
    let exec_env = ExecutorEnv::builder()
        .segment_limit_po2(po2)
        .build()?;
    let segment_callback = |_segment_info, _asset| -> anyhow::Result<()> {
        Ok(())
    };
    let session_info = r0_client.execute(
        &exec_env,
        Asset::Path(args.elf.into()),
        AssetRequest::Path(args.out_path.clone().into()),
        segment_callback,
    )?;
    let _ = std::fs::write(
        format!("{}/journal", args.out_path),
        bincode::serialize(
            &session_info.journal
        )?
    );
    println!("User cycles: `{}`, `{}` segments in total.",
        session_info.cycles(),
        session_info.segments.len()
    );
    Ok(())
}
