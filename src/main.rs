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
	fn id(&self) -> String;
    fn execute(&self, discord: &Discord, message: &Message, arguments: Vec<String>) -> String;
}

fn add_default_plugins<'a>(plugins: &mut Vec<Box<MPlugin>>) {
	plugins.push(Box::new(TextTransformPlugin));
	plugins.push(Box::new(ReactPlugin));
}

fn main() {
	
	let mut plugins: Vec<Box<MPlugin>> = Vec::new();
	add_default_plugins(&mut plugins);

	let token = read_file("res/token.txt");
	let catalyst = read_file("res/catalyst.txt");
	let discord = Discord::from_user_token(&token).expect(&format!("Invalid Token: {}", token));
	let (mut connection, ready) = discord.connect().expect("Connection failed.");
	loop {
	    match connection.recv_event() {
	    	Ok(Event::MessageCreate(ref mut message)) => {
	    		if message.author.id == ready.user.id {
		    		let ref m_content: String = message.content;
		    		if m_content.chars().take(catalyst.len()).collect::<String>() == catalyst {
		    			let ident = m_content.chars().skip(catalyst.len()).take_while(|&c| c != ' ').collect::<String>();
		    			for plugin in plugins.iter() {
		    				if ident.to_lowercase() == plugin.id() {
		    					let arguments = m_content.split_whitespace().skip(1).map(|x| String::from(x)).collect();
		    					discord.delete_message(message.channel_id, message.id).expect("Failed to delete message.");
		    					let result = &*&plugin.execute(&discord, message, arguments);
		    					if !result.is_empty() {
		    						discord.send_message(message.channel_id, result, "", false).expect("Failed to send message.");
		    					}
		    					break;
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