extern crate discord;
extern crate math_text_transform;

mod plugins;

use discord::Discord;
use discord::model::{Message, Event};

use plugins::*;

use std::fs::File;
use std::io::Read;



fn read_file(filename: &str) -> String {
	let mut file = File::open(filename).expect(&format!("File \"{}\" not found.", filename));
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect(&format!("Reading file \"{}\" failed.", filename));
	contents
}

pub trait MPlugin {
	fn id(&self) -> Vec<&str>;
   fn execute(&self, data: PluginData) -> String;
}

fn add_default_plugins<'a>(plugins: &mut Vec<Box<MPlugin>>) {
	plugins.push(Box::new(TextTransformPlugin));
	plugins.push(Box::new(ReactPlugin));
	plugins.push(Box::new(PurgePlugin));
	plugins.push(Box::new(SettingsPlugin));
	plugins.push(Box::new(UserInfoPlugin));
}

#[derive(Default)]
pub struct MisakiSettings {
	embed_mode:  bool,
	should_mark: bool
	
}

impl MisakiSettings {
	fn set(&mut self, name: &str, to: bool, flip: bool) -> Option<bool> {
		return match &*String::from(name).to_lowercase() {
			"embed" => { self.embed_mode = if flip { !self.embed_mode } else { to }; return Some(self.embed_mode) },
			"mark" => { self.should_mark = if flip { !self.should_mark } else { to }; return Some(self.should_mark) }, 
			_ => None
		}
	}
}


// used for taking up less space when passing information around to plugins
pub struct PluginData<'a> {
	discord:   &'a Discord,
	message:   &'a Message,
	arguments:     Vec<String>,
	settings:  &'a mut MisakiSettings,
}

fn main() {
	
	let mut plugins: Vec<Box<MPlugin>> = Vec::new();
	let mut settings: MisakiSettings = Default::default();
	add_default_plugins(&mut plugins);

	let token = read_file("res/token.txt");
	let catalyst = read_file("res/catalyst.txt");
	let discord = Discord::from_user_token(&token).expect(&format!("Invalid Token: {}", token));
	let (mut connection, ready) = discord.connect().expect("Connection failed.");
	loop {
	    match connection.recv_event() {
	    	Ok(Event::MessageCreate(ref message)) => {
	    		if message.author.id == ready.user.id {
		    		let ref m_content: String = message.content;
		    		if m_content.chars().take(catalyst.len()).collect::<String>() == catalyst {
		    			let ident = m_content.chars().skip(catalyst.len()).take_while(|&c| c != ' ').collect::<String>();
		    			'plugins: for plugin in plugins.iter() {
		    				'aliases: for alias in plugin.id() {
		    					if *&ident.to_lowercase() == alias {
			    					let arguments = m_content.split_whitespace().skip(1).map(|x| String::from(x)).collect();
			    					discord.delete_message(message.channel_id, message.id).expect("Failed to delete message.");
			    					let set = &mut settings; 
			    					let result = &*&plugin.execute(PluginData{discord: &discord, message: message, arguments: arguments, settings: set});
			    					if !result.is_empty() {
			    						discord.send_message(message.channel_id, &*format!("{} {}", if set.should_mark { "`►`" } else { "" }, result), "", false).expect("Failed to send message.");
			    					}
		    						break 'plugins;
		    					}
		    				}
		    			}
		    		}
	    		}
			}
	    	Ok(_) => {}
	        Err(discord::Error::Closed(code, body)) => {
	        	println!("Error: Gateway Closed. Code[{:?}] -- {}", code, body);
	        	break
	        }
	        Err(err) => println!("Got err: {}", err)
	    }
	}
}