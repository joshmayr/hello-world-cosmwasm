use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

use crate::msg::{ExecuteMsg, QueryMsg, GetMessageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HelloWorldContract(pub Addr);

impl HelloWorldContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    // Get Message
    pub fn read_message<Q, T, CQ>(&self, querier: &Q, index: u64) -> StdResult<GetMessageResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetMessage { index };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: GetMessageResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
