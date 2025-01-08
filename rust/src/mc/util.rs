use crate::{new_class, new_enum};

pub mod math;

new_enum! {ActionResult: "net/minecraft/util/ActionResult" {
    PASS: Pass,
    SUCCESS: Success
}}

new_class! {Hand: "net/minecraft/util/Hand" {}}
