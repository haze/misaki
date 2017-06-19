extern crate discord;
extern crate math_text_transform;
extern crate sharedlib as lib;
extern crate misaki_api;
extern crate glob;

use misaki_api::misaki::{MPlugin, MisakiSettings, PluginData};

mod plugins;

use discord::Discord;
use discord::model::{Message, Event};

use plugins::*;

use std::rc::Rc;
use std::fs::File;
use std::io::Read;

use glob::glob;

use lib::Symbol;
use lib::LibRc;

fn read_file(filename: &str) -> String {
	let mut file = File::open(filename).expect(&format!("File \"{}\" not found", filename));
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect(&format!("Reading file \"{}\" failed.", filename));
	contents
}

fn add_default_plugins(plugins: &mut Vec<(Option<LibRc>, Box<MPlugin>)>) {
	plugins.push((None, Box::new(TextTransformPlugin)));
	plugins.push((None, Box::new(ReactPlugin)));
	plugins.push((None, Box::new(PurgePlugin)));
	plugins.push((None, Box::new(SettingsPlugin)));
	plugins.push((None, Box::new(UserInfoPlugin)));
}


fn add_external_plugins(plugins: &mut Vec<(Option<LibRc>, Box<MPlugin>)>) {
	for dylib in glob("plugins/compiled/*.dylib").expect("Failed to read glob pattern...") {
		unsafe {
			let lib = LibRc::new(dylib.unwrap()).unwrap();
			let plugin: Box<MPlugin>;
			{
				let get_plugin_ex: lib::FuncTracked<fn() -> Box<MPlugin>, Rc<_>> = lib.find_func("get_plugin").unwrap();
				let plugin_sym = get_plugin_ex.get();
				let plugin_ptr: fn() -> Box<MPlugin> = std::mem::transmute(plugin_sym);
				plugin = plugin_ptr();	
			}
			plugins.push((Some(lib), plugin));
		}
	}
}

fn main() {
	
	let mut plugins: Vec<(Option<LibRc>, Box<MPlugin>)> = Vec::new();
	let mut settings: MisakiSettings = Default::default();
	
	add_default_plugins(&mut plugins);
	add_external_plugins(&mut plugins);
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
		    			'plugins: for tup in plugins.iter() {
		    				let (_, ref plugin) = *tup;
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