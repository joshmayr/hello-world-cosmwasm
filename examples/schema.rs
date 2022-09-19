use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use hello_world_cosmwasm::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, GetMessageResponse};
use hello_world_cosmwasm::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(GetCountResponse), &out_dir);
    export_schema(&schema_for!(GetMessageResponse), &out_dir);
}
