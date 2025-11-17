
use crate::abi::constant::{POOL_USDC_USDT_ADDRESS, USDC_ADDRESS, USDT_ADDRESS, MULLTICALL_ADDRESS};

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    sol_types::SolCall,
    transports::http::Http,
};
use eyre::Result;
use std::sync::Arc;
use crate::abi::{IMulticall, IUniswapv3pool, IERC20};
use crate::abi::Multicall3::{Call3};

pub async fn run(rpc_url: &str) -> Result<()> {

    let url = rpc_url.parse()?;
    let http = Http::new(url);
    let client = RpcClient::new(http, true);
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    let provider = Arc::new(provider);

    let mut calls = Vec::new();

    let pool_calls_data = vec![
        IUniswapv3pool::slot0Call {}.abi_encode(),
        IUniswapv3pool::liquidityCall {}.abi_encode(),
        IUniswapv3pool::token0Call {}.abi_encode(),
        IUniswapv3pool::token1Call {}.abi_encode(),
        IUniswapv3pool::feeCall {}.abi_encode(),
    ];


    for data in pool_calls_data {
        calls.push(Call3 {
            target: POOL_USDC_USDT_ADDRESS,
            allowFailure: true,
            callData: data.into(),
        });
    }


    let usdc_calls_data = vec![
        IERC20::nameCall{}.abi_encode(),
        IERC20::symbolCall {}.abi_encode(),
        IERC20::decimalsCall {}.abi_encode(),
        IERC20::totalSupplyCall {}.abi_encode(),
    ];

    for data in usdc_calls_data {
        calls.push(Call3 {
            target: USDC_ADDRESS,
            allowFailure: true,
            callData: data.into(),
        });
    }


    let usdt_calls_data = vec![
        IERC20::nameCall{}.abi_encode(),
        IERC20::symbolCall {}.abi_encode(),
        IERC20::decimalsCall {}.abi_encode(),
        IERC20::totalSupplyCall {}.abi_encode(),
    ];

    for data in usdt_calls_data {
        calls.push(Call3 {
            target: USDT_ADDRESS,
            allowFailure: true,
            callData: data.into(),
        });
    }

    let multicall = IMulticall::new(MULLTICALL_ADDRESS, provider.clone());

    let result = multicall
        .aggregate3(calls)
        .call().await?;

    let slot = IUniswapv3pool::slot0Call::abi_decode_returns(&result[0].returnData)?;
    let liquidity = IUniswapv3pool::liquidityCall::abi_decode_returns(&result[1].returnData)?;
    let token0_addr = IUniswapv3pool::token0Call::abi_decode_returns(&result[2].returnData)?;
    let token1_addr = IUniswapv3pool::token1Call::abi_decode_returns(&result[3].returnData)?;
    let pool_fee = IUniswapv3pool::feeCall::abi_decode_returns(&result[4].returnData)?;

    let usdc_name = IERC20::nameCall::abi_decode_returns(&result[5].returnData, )?;
    let usdc_symbol = IERC20::symbolCall::abi_decode_returns(&result[6].returnData, )?;
    let usdc_decimals = IERC20::decimalsCall::abi_decode_returns(&result[7].returnData, )?;
    let usdc_supply = IERC20::totalSupplyCall::abi_decode_returns(&result[8].returnData, )?;

    let usdt_name = IERC20::nameCall::abi_decode_returns(&result[9].returnData)?;
    let usdt_symbol = IERC20::symbolCall::abi_decode_returns(&result[10].returnData)?;
    let usdt_decimals = IERC20::decimalsCall::abi_decode_returns(&result[11].returnData)?;
    let usdt_supply = IERC20::totalSupplyCall::abi_decode_returns(&result[12].returnData)?;

    println!("\n===  Uniswap V3 Pool Info ===");
    println!("Address: {}", POOL_USDC_USDT_ADDRESS);
    println!("Fee: {} (0.01%)", pool_fee);
    println!("Liquidity: {}", liquidity);
    println!("SqrtPriceX96: {}", slot.sqrtPriceX96);
    println!("Tick: {}", slot.tick);

    println!("\n===  Token 0 (USDC) ===");
    println!("Address: {}", token0_addr);
    println!("Name: {}", usdc_name);
    println!("Symbol: {}", usdc_symbol);
    println!("Decimals: {}", usdc_decimals);
    println!("Total Supply: {}", usdc_supply);

    println!(" \n===Token 1 (USDT) ===");
    println!("Address: {}", token1_addr);
    println!("Name: {}", usdt_name);
    println!("Symbol: {}", usdt_symbol);
    println!("Decimals: {}", usdt_decimals);
    println!("Total Supply: {}", usdt_supply);

    Ok(())
}