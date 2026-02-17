use substreams_solana::pb::sol::v1::Block;
use substreams::errors::Error;

mod pb;
use pb::transfers::{Transfer, Transfers};

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    for confirmed_tx in block.transactions {
        let meta = match &confirmed_tx.meta { Some(m) => m, None => continue };
        if meta.err.is_some() { continue; }
        let tx = match &confirmed_tx.transaction { Some(t) => t, None => continue };
        let msg = match &tx.message { Some(m) => m, None => continue };
        let accounts = &msg.account_keys;
        for instruction in &msg.instructions {
            let data = &instruction.data;
            if data.len() == 12 && data[0..4] == [2, 0, 0, 0] {
                let amount = u64::from_le_bytes(data[4..12].try_into().unwrap_or([0u8; 8]));
                let from = accounts.get(instruction.accounts[0] as usize).map(|k| bs58::encode(k).into_string()).unwrap_or_default();
                let to = accounts.get(instruction.accounts[1] as usize).map(|k| bs58::encode(k).into_string()).unwrap_or_default();
                transfers.push(Transfer { from, to, amount });
            }
        }
    }
    Ok(Transfers { transfers })
}