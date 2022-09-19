#[cfg(test)]
mod tests {
    use crate::helpers::HelloWorldContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_hello_world() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, HelloWorldContract) {
        let mut app = mock_app();
        let hello_world_id = app.store_code(contract_hello_world());

        let msg = InstantiateMsg { count: 0i32 };
        let hello_world_contract_addr = app
            .instantiate_contract(
                hello_world_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let hello_world_contract = HelloWorldContract(hello_world_contract_addr);

        (app, hello_world_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn hello_world() {
            let (mut app, hello_world_contract) = proper_instantiate();

            let msg = ExecuteMsg::SendMessage { message: String::from("Hello, World!") };
            let cosmos_msg = hello_world_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
