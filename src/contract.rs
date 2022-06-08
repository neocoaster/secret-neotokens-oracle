use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage,
};

use crate::msg::{
    AddCreditsResponse, CountResponse, CreditsResponse, HandleMsg, InitMsg, QueryMsg,
};
use crate::state::{config, config_read, credits_storage, credits_storage_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetCredits { address } => to_binary(&try_get_credits(deps, address)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
) -> StdResult<CountResponse> {
    Ok(CountResponse { count: 10 })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Add { credits, address } => try_add_credits(deps, env, credits, address),
        HandleMsg::Reset {} => try_reset(deps, env),
    }
}

pub fn try_add_credits<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    credits: u64,
    address: HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    let receiver_address_canonical = deps.api.canonical_address(&address)?;
    let state = config_read(&deps.storage).load()?;

    if sender_address_raw != state.owner {
        return Err(StdError::Unauthorized { backtrace: None });
    }

    let _current_user_credits =
        match credits_storage_read(&deps.storage).load(receiver_address_canonical.as_slice()) {
            Ok(current_credits) => {
                if let Some(new_balance) = (current_credits).checked_add(credits) {
                    // TODO: IMPROVE ERROR HANDLING
                    let _credits = credits_storage(&mut deps.storage)
                        .save(receiver_address_canonical.as_slice(), &new_balance);
                } else {
                    return Err(StdError::generic_err("Balance would exceed limit"));
                }
            }
            Err(_error) => {
                let _credits = credits_storage(&mut deps.storage)
                    .save(receiver_address_canonical.as_slice(), &credits);
            }
        };

    // println!("{}", address.to_string());

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&AddCreditsResponse {
            receiver: address.to_string(),
            new_balance: 0,
        })?),
    })
}

fn try_get_credits<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
) -> StdResult<CreditsResponse> {
    let address_canonical = deps.api.canonical_address(&address)?;
    let current_user_credits =
        match credits_storage_read(&deps.storage).load(address_canonical.as_slice()) {
            Ok(current_credits) => current_credits,
            Err(_error) => 0,
        };

    Ok(CreditsResponse {
        owned_credits: current_user_credits,
    })
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    let state = config_read(&deps.storage).load()?;

    if sender_address_raw != state.owner {
        return Err(StdError::Unauthorized { backtrace: None });
    }

    // TODO: FLUSH STORAGE
    // credits_storage(&mut deps.storage).clear();

    debug_print("credits reset successfully");
    Ok(HandleResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        // println!("INIT TEST");
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&mut deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(10, value.count);
    }

    #[test]
    fn add_credits() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        let env = mock_env("creator", &coins(2, "token"));
        let querier_env = mock_env("demian", &coins(2, "token"));
        let msg = HandleMsg::Add {
            credits: 7,
            address: querier_env.message.sender,
        };
        let _res = handle(&mut deps, env, msg).unwrap();

        let querier_env = mock_env("demian", &coins(2, "token"));
        let res = query(
            &mut deps,
            QueryMsg::GetCredits {
                address: querier_env.message.sender,
            },
        )
        .unwrap();
        let value: CreditsResponse = from_binary(&res).unwrap();
        assert_eq!(7, value.owned_credits);
    }

    // #[test]
    // fn reset() {
    //     PENDING
    // }
}
