use bitcoincore_rpc::bitcoin::BlockHash;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Providing Block height
    #[arg(long)]
    pub blkheight: Option<u64>,

    /// Providing block hash
    #[arg(long)]
    pub blkhash: Option<BlockHash>,
}
