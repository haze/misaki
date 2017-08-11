extern crate eval;
extern crate misaki_api;

use misaki_api::misaki::{PluginData, MPlugin};
use eval::{eval, to_value, };
use std::error::Error;


#[no_mangle]
pub fn get_plugin() -> Box<MPlugin> { Box::new(EvalPlugin) }

struct EvalPlugin;
impl MPlugin for EvalPlugin {
	fn id(&self) -> Vec<&str> { vec!("eval", "e") }
	fn execute(&self, data: PluginData) -> String {
		return match eval(&data.arguments.join(" ")) {
		    Ok(val) => to_value(val).to_string(),
		    Err(why) => String::from(why.description()),
		}
	}
}