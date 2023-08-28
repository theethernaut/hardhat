use crate::U256;

/// Test whether a block number is safe from a reorg for a specific chain based on the latest block
/// number.
pub fn is_safe_block_number(args: IsSafeBlockNumberArgs<'_>) -> bool {
    let safe_block_number = largest_safe_block_number((&args).into());
    args.block_number <= &safe_block_number
}

/// Arguments for the `is_safe_block_number` function.
/// The purpose of this struct is to prevent mixing up the U256 arguments.
pub struct IsSafeBlockNumberArgs<'a> {
    /// The chain id
    pub chain_id: &'a U256,
    /// The latest known block number
    pub latest_block_number: &'a U256,
    /// The block number to test
    pub block_number: &'a U256,
}

impl<'a> From<&'a IsSafeBlockNumberArgs<'a>> for LargestSafeBlockNumberArgs<'a> {
    fn from(value: &'a IsSafeBlockNumberArgs<'a>) -> LargestSafeBlockNumberArgs<'a> {
        LargestSafeBlockNumberArgs {
            chain_id: value.chain_id,
            latest_block_number: value.latest_block_number,
        }
    }
}

/// The largest block number that is safe from a reorg for a specific chain based on the latest
/// block number.
pub fn largest_safe_block_number(args: LargestSafeBlockNumberArgs<'_>) -> U256 {
    args.latest_block_number
        .saturating_sub(largest_possible_reorg(args.chain_id))
}

/// Arguments for the `largest_safe_block_number` function.
/// The purpose of this struct is to prevent mixing up the U256 arguments.
pub struct LargestSafeBlockNumberArgs<'a> {
    /// The chain id
    pub chain_id: &'a U256,
    /// The latest known block number
    pub latest_block_number: &'a U256,
}

/// Retrieves the largest possible size of a reorg, i.e. ensures a "safe" block.
///
/// # Source
///
/// The custom numbers were taken from:
/// <https://github.com/NomicFoundation/hardhat/blob/caa504fe0e53c183578f42d66f4740b8ec147051/packages/hardhat-core/src/internal/hardhat-network/provider/utils/reorgs-protection.ts>
pub fn largest_possible_reorg(chain_id: &U256) -> U256 {
    let chain_id: u64 = chain_id.try_into().expect("invalid chain id");
    let threshold: u64 = match chain_id {
        // Ropsten
        3 => 100,
        // xDai
        100 => 38,
        // One epoch on Ethereum mainnet
        _ => 32,
    };
    U256::from(threshold)
}
