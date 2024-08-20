use cielo_rs_interface::*;

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     env_logger::init(); // enables info! for debugging, used in lib.rs in URL construction

     let request = CieloRequest {
          wallet: Some("GTdu7yv9DefWrEoWZnRc744qMEo5DFgrrdar7QdivEwf".to_string()),
          limit: None,
          list: None,
          chains: Some(vec![Chain::Solana, Chain::Ethereum, Chain::EvmChain("polygon".to_string())]),
          types: Some(vec![TxType::Swap, TxType::Transfer, TxType::Lp, TxType::Staking]),
          tokens: None,
          min_usd: None,
          new_trades: Some(true),
          start_from: None,
          from_timestamp: None,
          to_timestamp: None,
     };

     let response = 
          submit_cielo_get_request(request).await;

    match response {
        Ok(body) => {
            println!("{}", body);
        }
        Err(e) => {
            eprintln!("Error fetching transactions: {}", e);
        }
    }

    Ok(())
}