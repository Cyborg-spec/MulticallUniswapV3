pub mod constant;
use alloy::sol;
sol!(
    IUniswapv3pool,
    "src/abi/UniswapV3Pool.json"
);
sol!(
     #[sol(rpc)]
     IMulticall,
    "src/abi/Multicall.json"
);
sol!(
    IERC20,
    "src/abi/ERC20.json"
);
