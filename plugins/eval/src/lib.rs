extern crate misaki_api;
use misaki_api::misaki::{PluginData, MPlugin};

#[no_mangle]
pub fn get_plugin() -> Box<MPlugin> { Box::new(EvalPlugin) }

struct EvalPlugin;
impl MPlugin for EvalPlugin {
	fn id(&self) -> Vec<&str> { vec!("eval", "e") }
	fn execute(&self, data: PluginData) -> String {
		String::new()
	}
}