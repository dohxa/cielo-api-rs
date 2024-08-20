use reqwest::Client;
use reqwest::header::ACCEPT;
use std::error::Error;
use serde::{Serialize, Deserialize};
use log::info;

// API KEY
pub const API_KEY: &str = "";

/*
    Base URL for making GET requests to the Cielo API
*/
pub const BASE_URL: &str = "https://feed-api.cielo.finance/api/v1/feed?";

/*
    Enum that represents the different tx types that are able to be queried
    from the Cielo API. Use TxType::to_get_format() to get the string 
    representation of the enum in the API's GET request.
 */
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TxType {
    Bridge,
    ContractCreation,    ContractInteraction,
    Flashloan,
    Lending,
    Lp,
    NftLending,
    NftLiquidation,
    NftMint,
    NftSweep,
    NftTrade,
    NftTransfer,
    Option,
    Perp,
    Reward,
    Staking,
    SudoPool,
    Swap,
    Transfer,
    Wrap
}

impl TxType {
    fn to_get_format(&self) -> &str {
        match self {
            TxType::Bridge => "bridge",
            TxType::ContractCreation => "contract_creation",
            TxType::ContractInteraction => "contract_interaction",
            TxType::Flashloan => "flashloan",
            TxType::Lending => "lending",
            TxType::Lp => "lp",
            TxType::NftLending => "nft_lending",
            TxType::NftLiquidation => "nft_liquidation",
            TxType::NftMint => "nft_mint",
            TxType::NftSweep => "nft_sweep",
            TxType::NftTrade => "nft_trade",
            TxType::NftTransfer => "nft_transfer",
            TxType::Option => "option",
            TxType::Perp => "perp",
            TxType::Reward => "reward",
            TxType::Staking => "staking",
            TxType::SudoPool => "sudo_pool",
            TxType::Swap => "swap",
            TxType::Transfer => "transfer",
            TxType::Wrap => "wrap",
        }
    }
}

/*
    Enum for the different chains that can be queried from. For EVM based chains,
    such as Polygon, represent as such: EvmChain("polygon")
*/
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Chain {
    Solana,
    Ethereum,
    EvmChain(String)
}

impl Chain {
    fn to_get_format(&self) -> &str {
        match self {
            Chain::Solana => "solana",
            Chain::Ethereum => "ethereum",
            Chain::EvmChain(chain) => chain.as_str(),
        }
    }
}

/*
    This struct represents the data that can be specified in a request to 
    the Cielo API.
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CieloRequest {
    pub wallet: Option<String>,                          // wallet address
    pub limit: Option<usize>,                             // limit # of tx returned in the request, max is 100
    pub list: Option<usize>,                               // filter tx by a specific list id
    pub chains: Option<Vec<Chain>>,            // filter tx by specific chains
    pub types: Option<Vec<TxType>>,           // filter tx by a specific tx type
    pub tokens: Option<Vec<String>>,          // filter tx by specific tokens, identified by either their address or symbol
    pub min_usd: Option<usize>,                   // minimum usd value for the transactions
    pub new_trades: Option<bool>,               // filter transactions by new trades
    pub start_from: Option<String>,             // ?????
    pub from_timestamp: Option<i64>,       // from timestamp utc
    pub to_timestamp: Option<i64>,           // to timestamp utc
}

/*
    passing a CieloRequest object to this function will return
    data from the Cielo API corresponding to the information
    specified in the CieloRequest object.
*/
pub async fn submit_cielo_get_request(req: CieloRequest) ->
Result<String, Box<dyn Error + Send + Sync>> {
    let client = Client::new();

    let mut chains_string = String::new();
    let mut index = 0;
    for chain in &req.chains.clone().unwrap() {
        if index != req.chains.clone().unwrap().len() {
            chains_string.push_str(chain.to_get_format());
            chains_string.push(',');
        } else {
            chains_string.push_str(chain.to_get_format());
        }
        index += 1;
    }

    let mut tx_type_string = String::new();
    index = 0;
    for tx_type in &req.types.clone().unwrap() {
        if index!= req.types.clone().unwrap().len() {
            tx_type_string.push_str(tx_type.to_get_format());
            tx_type_string.push('&');
        } else {
            tx_type_string.push_str(tx_type.to_get_format());
        }
        index += 1;
    }

    let mut _q_url = 
        construct_url_from_req_object(req)
        .await
        .expect("Error constructing GET request query values (URL)");

    let response = client
        .get(_q_url)
        .header(ACCEPT, "application/json")
        .header("X-API-KEY", API_KEY)
        .send().await?;

    let body = response.text().await?;
    println!("{}", body);   

    Ok(body)
}

async fn construct_url_from_req_object(request: CieloRequest) -> Result<String, Box<dyn Error>> {
    let mut _q_url = String::new();
    
    // Add base URL to the constructed URL
    _q_url.push_str(BASE_URL);

    // wallet address
    if let Some(wallet) = request.wallet {
        let url_slice: String = format!("wallet={}", wallet);
        _q_url.push_str(&url_slice);
        info!("URL CONSTRUCTOR:\n \x1b[35mAdding wallet address:\x1b[0m {}", _q_url);
    } else {
        return Err("Wallet address is required".into());
    }

    // tx limit
    if let Some(limit) = request.limit {
        let url_slice: String = format!("&limit={}", limit);
        _q_url.push_str(&url_slice);
        info!("URL CONSTRUCTOR:\n \x1b[35mAdding tx limit:\x1b[0m {}", _q_url);
    }

    // list id
    if let Some(list) = request.list {
        let url_slice: String = format!("&list={}", list);
        _q_url.push_str(&url_slice);
        info!("URL CONSTRUCTOR:\n \x1b[35mAdding list ID:\x1b[0m {}", _q_url);
    }

    // chains 
    if let Some(chains) = &request.chains { 
        
        let mut index: usize = 0;
        for chain in chains {
            if index == 0 {
                let url_slice: String = format!("&chains={}", chain.to_get_format());
                _q_url.push_str(&url_slice);
            } else {
                let url_slice: String = format!(",{}", chain.to_get_format());
                _q_url.push_str(&url_slice);
            }

            index += 1;
        }  

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding chains:\x1b[0m {}", _q_url);
    }

    // tx types
    if let Some(types) = &request.types {

        let mut index: usize = 0;
        for tx_type in types {
            if index == 0 {
                let url_slice: String = format!("&txTypes={}", tx_type.to_get_format());
                _q_url.push_str(&url_slice);
            } else {
                let url_slice: String = format!(",{}", tx_type.to_get_format());
                _q_url.push_str(&url_slice);
            }

            index += 1;
        }

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding tx types:\x1b[0m {}", _q_url);
    }

    // tokens
    if let Some(tokens) = &request.tokens {

        let mut index: usize = 0;
        for token in tokens {
            if index == 0 {
                let url_slice: String = format!("&tokens={}", token);
                _q_url.push_str(&url_slice);
            } else {
                let url_slice: String = format!(",{}", token);
                _q_url.push_str(&url_slice);
            }

            index += 1;
        }

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding tokens:\x1b[0m {}", _q_url);
    }

    // min usd
    if let Some(min_usd) = request.min_usd {
        let url_slice: String = format!("&minUSD={}", min_usd);
        _q_url.push_str(&url_slice);

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding min USD amount for txs:\x1b[0m {}", _q_url);
    }

    // new trades
    if let Some(new_trades) = request.new_trades {
        if new_trades {
            _q_url.push_str("&newTrades=true");
        } else {
            _q_url.push_str("&newTrades=false");
        }

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding new trades filter:\x1b[0m {}", _q_url);
    }  

    // start from
    if let Some(start_from) = &request.start_from {
        let url_slice = format!("&startFrom={}", start_from);
        _q_url.push_str(&url_slice);

        info!(
            "URL CONSTRUCTOR:\n \x1b[35mstartFrom value for response `paging.next_object_id` to get next page:\x1b[0m {}",
             _q_url
        );
    }

    // from timestamp
    if let Some(from_timestamp) = request.from_timestamp {
        let url_slice = format!("&fromTimestamp={}", from_timestamp);
        _q_url.push_str(&url_slice);

        info!("URL CONSTRUCTOR:\n \x1b[35mAdding from_timestamp (UTC):\x1b[0m {}", _q_url);
    }

    // to timestamp
    if let Some(to_timestamp) = request.to_timestamp {
        let url_slice = format!("&toTimestamp={}", to_timestamp);
        _q_url.push_str(&url_slice);
        
        info!("URL CONSTRUCTOR:\n \x1b[35mAdding to_timestamp (UTC):\x1b[0m {}", _q_url);
    }

    Ok(_q_url)
}


#[cfg(test)]
mod tests {
    use super::*;

    // eth test
    #[tokio::test]
    async fn it_gets_transactions() {
        let request = CieloRequest {
            wallet: Some("0x0f9d76acdbc4417b026f876be1e2042e45f3bcd2".to_string()),
            limit: Some(10),
            list: None,
            chains: Some(vec![Chain::Ethereum]),
            types: Some(vec![TxType::Swap]),
            tokens: None,
            min_usd: Some(100),
            new_trades: None,
            start_from: None,
            from_timestamp: None,
            to_timestamp: None,
        };

        submit_cielo_get_request(request).await.expect("Error getting transactions");
    }

    // sol test
    #[tokio::test]
    async fn it_gets_sol_txs() {
        let request = CieloRequest {
            wallet: Some("GTdu7yv9DefWrEoWZnRc744qMEo5DFgrrdar7QdivEwf".to_string()),
            limit: Some(10),
            list: None,
            chains: Some(vec![Chain::Solana]),
            types: Some(vec![TxType::Swap, TxType::Transfer]),
            tokens: None,
            min_usd: Some(100),
            new_trades: None,
            start_from: None,
            from_timestamp: None,
            to_timestamp: None,
        };

        submit_cielo_get_request(request).await.expect("Error getting transactions");
    }
    
}
