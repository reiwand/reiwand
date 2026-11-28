#![allow(dead_code)]

use serde_json::Value;

pub fn scene(text: &str) -> String {
    reiwand_fill::whole(text)
}

pub fn descriptor(mut descriptor: Value) -> String {
    reiwand_fill::descriptors::complete(&mut descriptor);
    descriptor.to_string()
}

pub fn manifest(text: &str) -> String {
    reiwand_fill::manifests::whole(text)
}
