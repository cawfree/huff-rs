#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

//use huff_core::Compiler;
use huff_utils::{abi::Abi, artifact::Artifact, error::CompilerError, prelude::EVMVersion};

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let data = vec![0x83, b'c', b'a', b't'];
    let _animal: String = rlp::decode(&data).unwrap();
    // assert_eq!(animal, "cat".to_owned());

    0
}
