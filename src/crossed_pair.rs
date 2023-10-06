#![allow(unused)]
use crate::address_book::{UniQuery, MevBot, UNISWAP_ROUTER, SUSHISWAP_ROUTER, WETH_ADDRESS, MEV_ADDRESS};
use crate::utils::*;
use ethers::{abi::ethereum_types::U512, prelude::*, utils::{format_ether, parse_ether}};

#[derive(Debug)]
pub struct CrossedPairManager<'a, M>
where
    M: Middleware,
{
    flash_query_contract: &'a UniQuery<M>,
    markets: Vec<TokenMarket<'a>>,
}

impl<'a, M> CrossedPairManager<'a, M>
where
    M: Middleware,
{
    pub fn new(
        grouped_pairs: &'a [(H160, Vec<[H160; 3]>)],
        flash_query_contract: &'a UniQuery<M>,
    ) -> Self {
        let pairs = grouped_pairs
            .iter()
            .map(|(token, pairs)| TokenMarket {
                token,
                pairs: pairs
                    .iter()
                    .copied()
                    .map(|[token0, token1, address]| Pair {
                        address,
                        token0,
                        token1,
                        reserve: None,
                    })
                    .collect::<Vec<Pair>>(),
            })
            .collect::<Vec<TokenMarket>>();
        Self {
            markets: pairs,
            flash_query_contract,
        }
    }

    pub fn write_tokens(&mut self) {
        let tokens: Vec<H160> = self.markets.iter().map(|market| *market.token).collect();
        dbg!(tokens.len());
        if let Err(e) = write_tokens_to_file(tokens.clone()) {
            eprintln!("Failed to write tokens to file: {}", e);
            }
        }

    pub async fn update_reserve(&mut self) {
        let reserves = self
            .get_all_pair_addresses()
            .iter()
            .map(|pair| pair.address)
            .collect::<Vec<H160>>();

        let reserves = self
            .flash_query_contract
            .get_reserves_by_pairs(reserves)
            .call()
            .await
            .unwrap();

        let min_weth = parse_ether(500).unwrap();   // Filter out pairs that have more than 500 WETH

        for (new_reserve, pair) in std::iter::zip(&reserves, self.get_all_pair_addresses()) {
            let weth_address = &WETH_ADDRESS.parse::<Address>().unwrap();
            let (reserve0, reserve1) = if &pair.token0 == weth_address {
                (new_reserve[1], new_reserve[0])
            } else {
                (new_reserve[0], new_reserve[1])
            };

            if reserve0 >= min_weth || reserve1 >= min_weth {
                let updated_reserve = Reserve {
                    reserve0,
                    reserve1,
                    // block_timestamp_last: new_reserve[2],
                };
    
                pair.reserve = Some(updated_reserve);
            } else {
                pair.reserve = None;
            }
        }
        for market in &mut self.markets {
            market.pairs.retain(|pair| pair.reserve.is_some());
        }
    }

    pub fn get_all_pair_addresses(&mut self) -> Vec<&mut Pair> {
        self.markets
            .iter_mut()
            .flat_map(|token_market| &mut token_market.pairs)
            .collect::<Vec<&mut Pair>>()
    }

    pub fn find_arbitrage_opportunities(&mut self) {
        for market in &mut self.markets {
            market.find_arbitrage_opportunity();
        }
    }
}


#[derive(Debug)]
pub struct TokenMarket<'a> {
    token: &'a H160,
    pairs: Vec<Pair>,
}

impl<'a> TokenMarket<'a> {
    pub fn find_arbitrage_opportunity(&self) {
        for pair_a in &self.pairs {
            for pair_b in &self.pairs {
                if let Some((x, _alt_amount, profit)) = profit(
                    pair_a.reserve.as_ref().unwrap(),
                    pair_b.reserve.as_ref().unwrap(),
                ) {
                    if profit.gt(&U512::from(parse_ether(0.01).unwrap())) {  
                        let token = *self.token;
                        let pair1 = pair_a.address;
                        let pair2 = pair_b.address;
                        let eth1 = parse_ether(1).unwrap();  // Send 1 WETH

                        // Since I found the "profit" function wasn't always accurate,
                        // I included a second calculate function to confirm if the
                        // path is profitable or not, if it is, we execute it.
                        tokio::spawn(async move {       
                                let a = calculate(token, eth1).await;
                                match a {
                                    Some(a) => {
                                        if a.gt(&eth1) {
                                            println!("\n---------------------------------- ATTEMPTING ARB ----------------------------------------");
                                            dbg!(token, pair1, pair2);
                                            println!("------------------------------------------------------------------------------------------");
                                            execute(token, eth1).await;     // Execute function
                                        }
                                    },
                                    None => {
                                    }
                                }
                            });

                    }
                }
            }
        }
    }
}


use std::{sync::Arc, ops::Add};

pub async fn calculate(
    token_2: H160,
    amount: U256
    ) -> Option<U256> {
    
    let config = Config::new().await;
    let contract_addr = address(MEV_ADDRESS);
    let mevbot = MevBot::new(
        contract_addr, 
        Arc::clone(&config.http)
    );

    let router_1 = address(UNISWAP_ROUTER);
    let router_2 = address(SUSHISWAP_ROUTER);
    let token_1 = address(WETH_ADDRESS);

    let calculate: U256 = mevbot
    .estimate_dual_dex_trade(router_1, router_2, token_1, token_2, amount)
    .call()
    .await
    .unwrap();

    let gas_price = config.http.get_gas_price().await.unwrap();
    if calculate.gt(&amount.add(gas_price)) {
        Some(calculate)
    } else {
        None 
    }
}

pub async fn execute(
    token_2: H160,
    amount: U256
    ) {
    let config = Config::new().await;
    let contract_addr = address(MEV_ADDRESS);
    let mevbot = MevBot::new(
        contract_addr, 
        Arc::clone(&config.http)
    );

    let router_1 = address(UNISWAP_ROUTER);
    let router_2 = address(SUSHISWAP_ROUTER);
    let token_1 = address(WETH_ADDRESS);

    let trade = match mevbot.dual_dex_trade(router_1, router_2, token_1, token_2, amount)
    .send()
    .await {
        Ok(c) => {eprintln!("\nArbitrage completed : {:?}\n", c)},
        Err(e) => {
            //eprintln!("\nArbitrage failed : {:?}\n", e);
        }
    };
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Pair {
    address: H160,
    token0: H160,
    token1: H160,
    reserve: Option<Reserve>,
}

#[derive(Debug)]
pub struct Reserve {
    reserve0: U256,
    reserve1: U256,
}

impl Reserve {
    pub fn new(reserve0: U256, reserve1: U256) -> Self {
        Self { reserve0, reserve1 }
    }
}

pub fn profit(pair_a: &Reserve, pair_b: &Reserve) -> Option<(U512, U512, U512)> {
    let q = U512::from(pair_a.reserve0 * pair_b.reserve1);
    let r = U512::from(pair_b.reserve0 * pair_a.reserve1);
    let s = U512::from(pair_a.reserve0 + pair_b.reserve0);
    if r > q {
        return None;
    }

    let r2 = r.checked_pow(U512::from(2i32)).expect("power overflow");
    let x_opt = (r2 + ((q * r - r2) / s)).integer_sqrt() - r;
    if x_opt == U512::from(0u128) {
        return None;
    }
    let alt_amount = U512::from(pair_a.reserve0) * x_opt / (U512::from(pair_a.reserve1) + x_opt);
    let p = (q * x_opt) / (r + s * x_opt) - x_opt;

    Some((x_opt, alt_amount, p))
}

use std::fs::File;
use std::io::prelude::*;
fn write_tokens_to_file(tokens: Vec<H160>) -> std::io::Result<()> {
    let mut file = File::create("src/tokens.txt")?;
    let tokens_str = tokens
        .iter()
        .map(|token| format!("\"{:?}\"", token))
        .collect::<Vec<String>>()
        .join(", ");
    let tokens_line = format!("[{}]", tokens_str);
    writeln!(file, "{}", tokens_line)?;

    Ok(())
}
