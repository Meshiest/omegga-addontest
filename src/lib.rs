#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod omegga;

use napi::{Env, JsObject};
pub use omegga::*;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn get_version(env: Env, omegga: JsObject) {
  let omegga = match Omegga::new(omegga, env) {
    Ok(omegga) => omegga,
    Err(e) => {
      println!("{e:?}");
      return;
    }
  };

  omegga.writeln(&format!("Chat.Broadcast \"{}\"", omegga.get_version()))
}
