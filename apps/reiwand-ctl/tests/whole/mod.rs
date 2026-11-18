#![allow(dead_code)]

use serde_json::Value;

pub fn scene(text: &str) -> String {
    reiwand_upgrade::fill::whole(&text.replace("reiwand.scene/1", reiwand_scene::SCHEMA))
}

pub fn descriptor(mut descriptor: Value) -> String {
    reiwand_upgrade::descriptors::complete(&mut descriptor);
    descriptor.to_string()
}
