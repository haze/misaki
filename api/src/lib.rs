#![allow(dead_code)]
extern crate discord;

pub mod misaki {

   use discord::Discord;
   use discord::model::Message;

	#[derive(Default)]
	pub struct MisakiSettings {
		pub embed_mode:  bool,
		pub should_mark: bool
	}

	impl MisakiSettings {
		pub fn set(&mut self, name: &str, to: bool, flip: bool) -> Option<bool> {
			return match &*String::from(name).to_lowercase() {
				"embed" => { self.embed_mode = if flip { !self.embed_mode } else { to }; return Some(self.embed_mode) },
				"mark" => { self.should_mark = if flip { !self.should_mark } else { to }; return Some(self.should_mark) }, 
				_ => None
			}
		}
	}


	// used for taking up less space when passing information around to plugins
	pub struct PluginData<'a> {
		pub discord:   &'a Discord,
		pub message:   &'a Message,
		pub arguments:     Vec<String>,
		pub settings:  &'a mut MisakiSettings,
	}
	
	pub trait MPlugin {	
		fn id(&self) -> Vec<&str>;
	  	fn execute(&self, data: PluginData) -> String;
	}

}