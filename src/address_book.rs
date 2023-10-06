use ethers::prelude::*;

pub const UNISWAP_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
pub const UNISWAP_FACTORY: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

pub const SUSHISWAP_ROUTER: &str = "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F";
pub const SUSHISWAP_FACTORY: &str = "0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac";

pub const QUERY_CONTRACT: &str = "0x5EF1009b9FCD4fec3094a5564047e190D72Bd511";
pub const MEV_ADDRESS: &str = "0x0000000000000000000000000000000000000000";     // REPLACE WITH YOUR OWN DEPLOYED CONTRACT

pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

abigen!(UniV2Router, "src/abi/UniV2Router.json");
abigen!(UniV2Factory, "src/abi/UniV2Factory.json");
abigen!(UniV2Pair, "src/abi/UniV2Pair.json");

abigen!(UniQuery, "src/abi/UniQuery.json");
abigen!(MevBot, "src/abi/MEV.json");