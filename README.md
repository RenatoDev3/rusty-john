# Dex Arbitrage - MEV Bot  ![license](https://img.shields.io/badge/License-MIT-green.svg?label=license)
Open sourcing a MEV Arbitrage Bot written in blazing fast Rust.

## Before Starting
This repo should be used as reference material for aspiring searchers looking to get into the world of MEV.
I removed more complex functions, mechanisms & optimizations to make it simple and easy to follow, you won't get all the alpha :)

Although this bot should only be used as reference material on how to perform Arbitrage, I found it to be profitable on lesser known EVM compatible chains where there isn't much competition.
This bot wouldn't be profitable on a competitive EVM chain like Ethereum, BSC or Solana.

That being said, use it at your own risk, I don't guarantee any gains, you will lose money. Use this repo as a way to gain practical knowledge, not to try and make money.

I am making my work public as it is very hard to get into the world of MEV, 
there is very limited information on the topic and it's hard to know where to start. Not only do I want people to learn from this repo, 
but I also want to inspire others to get into the world of MEV and better understand it.

Happy searching!

## Features
The code is simplified quite a bit, it currently sends 1 ETH when a profitable arbitrage path is found, but there is an optimal input amount function.
- [x] Query Uniswap pairs
- [x] Find matching pairs across different exchanges
- [x] Update pair reserves
- [x] Find arbitrage opportunities

## Examples
Besides the `crossed_pairs` and `query_test` functions that are used to test the code. In the "*examples*" folder I included multiple useful functions for testing, including:
- [x] View pending transactions in the mempool
- [x] View pending Uniswap V2 transactions in the mempool
- [x] Subscribe blocks

Any of these functions can be run using: `cargo run --example <name of file>`.

## Improvements
If you wish to contribute to the repo, some features that could be implemented are:
- A better optimal profit function
- Estimate an array of profitable tokens instead of 1 by 1
- Make the execute function more gas efficient

## Notice
If any bugs or optimizations are found, feel free to create a pull request. **All pull requests are welcome!** 

> **Warning**
>
> **This software is highly experimental and should be used at your own risk.** Although tested, this bot is experimental software and is provided on an "as is" and "as available" basis under the MIT license. We cannot guarantee the stability or reliability of this codebase and are not responsible for any damage or loss caused by its use. We do not give out warranties. 
