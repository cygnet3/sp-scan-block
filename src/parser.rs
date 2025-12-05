use bitcoincore_rpc::bitcoin::{BlockHash, Txid};
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

    /// Providing txid for singular tx
    #[arg(long)]
    pub txid: Option<Txid>,
}
