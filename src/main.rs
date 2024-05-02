mod config;
mod parser;

use std::process::exit;

use bitcoincore_rpc::RpcApi;
use clap::Parser;
use silentpayments::{
    secp256k1::PublicKey,
    utils::receiving::{calculate_tweak_data, get_pubkey_from_input},
};

use crate::{config::get_rpc_client_from_config, parser::Args};

fn main() {
    let args = Args::parse();
    let client = get_rpc_client_from_config();

    let blk_hash = match (args.blkhash, args.blkheight) {
        (Some(_), Some(_)) => {
            println!("specify either block hash or height");
            exit(1);
        }
        (Some(blk_hash), _) => blk_hash,
        (_, Some(blk_height)) => client.get_block_hash(blk_height).unwrap(),
        (None, None) => {
            println!("specify either block hash or height");
            exit(1);
        }
    };

    let block = client.get_block(&blk_hash).unwrap();

    let mut tweaks = Vec::new();

    for tx in block.txdata {
        // skip coinbase tx
        if tx.is_coinbase() {
            continue;
        }

        // skip tx if there are no taproot outputs
        let contains_taproot = tx.output.iter().any(|txout| txout.script_pubkey.is_p2tr());
        if !contains_taproot {
            continue;
        }

        let mut pubkeys: Vec<PublicKey> = Vec::with_capacity(tx.input.len());
        let mut outpoints: Vec<(String, u32)> = Vec::with_capacity(tx.input.len());
        for txin in tx.input {
            let prevout = txin.previous_output;
            outpoints.push((prevout.txid.to_string(), prevout.vout));

            // get signature and witness from txin
            let script_sig = txin.script_sig.to_bytes();
            let witness = txin.witness.to_vec();

            // get scriptpubkey from previous tx
            let prev_tx = client.get_raw_transaction(&prevout.txid, None).unwrap();
            let prev_tx_out = prev_tx.output.get(prevout.vout as usize).unwrap();
            let prevout_spk = prev_tx_out.script_pubkey.to_bytes();

            // check if this input is sp-eligible
            match get_pubkey_from_input(&script_sig, &witness, &prevout_spk).unwrap() {
                Some(pubkey) => pubkeys.push(pubkey),
                None => (),
            }
        }

        let pubkeys_ref: Vec<&PublicKey> = pubkeys.iter().collect();

        if !pubkeys_ref.is_empty() {
            let tweak = calculate_tweak_data(&pubkeys_ref, &outpoints).unwrap();
            tweaks.push(tweak);
        }
    }

    let tweaks_strs: Vec<_> = tweaks.into_iter().map(|pk| pk.to_string()).collect();

    println!("{:#?}", tweaks_strs);
}
