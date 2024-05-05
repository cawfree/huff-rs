#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(where_clauses_object_safety)]

use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use huff_core::Compiler;
use huff_utils::{abi::Abi, artifact::Artifact, error::CompilerError, prelude::EVMVersion};
use zkwasm_rust_sdk::wasm_output;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::wasm_dbg;
use zkwasm_rust_sdk::require;

use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct CompilerInput {
    evm_version: Option<String>,
    sources: Vec<String>,
    files: HashMap<String, String>,
    construct_args: Option<Vec<String>>,
    alternative_main: Option<String>,
    alternative_constructor: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CompilerArtifact {
    bytecode: String,
    runtime: String,
    abi: Option<Abi>,
}

#[derive(Serialize, Deserialize)]
struct CompilerOutput {
    errors: Option<Vec<String>>,
    contracts: Option<HashMap<String, CompilerArtifact>>,
}

#[wasm_bindgen]
pub fn zkmain() {

    let mut s = String::new();
    let len = unsafe {wasm_input(0)};

    for i in 0..len {
        let val = unsafe {wasm_input((i + 1).try_into().unwrap())};
        s.push(char::from_u32(val.try_into().unwrap()).unwrap());
    }

    let input: CompilerInput = serde_json_wasm::from_str(&s).unwrap();

    let evm_version = EVMVersion::from(input.evm_version);

    // TODO: need getrandom
    //let compiler = Compiler::new_in_memory(
    //    &evm_version,
    //    Arc::new(input.sources),
    //    input.files,
    //    input.alternative_main,
    //    input.alternative_constructor,
    //    input.construct_args,
    //    None,
    //    false,
    //);

//     let input: CompilerInput = serde_wasm_bindgen::from_value(s.into()).unwrap();

    //let commands_len = unsafe {wasm_input(0)};
    //let data = vec![0x83, b'c', b'a', b't'];
    //let _animal: String = rlp::decode(&data).unwrap();

    // assert_eq!(animal, "cat".to_owned());
    //0
}
