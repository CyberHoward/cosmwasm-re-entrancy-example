use cosmwasm_std::{Deps, StdResult};

use crate::msg::FactoriesResponse;
use crate::state::{read_factories, ConfigResponse, CONFIG};

pub fn query_factories(deps: Deps, limit: Option<u32>) -> StdResult<FactoriesResponse> {
    let factories = read_factories(deps, limit)?;
    Ok(FactoriesResponse { factories })
}
/// Queries the [Config], which contains the owner address
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}
