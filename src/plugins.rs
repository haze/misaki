extern crate discord;		
extern crate math_text_transform;

use math_text_transform::MathTextTransform;
use MPlugin;
use discord::model::ReactionEmoji;
use discord::model::Message;
use discord::model::Channel::*;
use discord::Discord;

pub struct ReactPlugin;
impl MPlugin for ReactPlugin {
	fn id(&self) -> String { String::from("react") }
	fn execute(&self, d: &Discord, msg: &Message, args: Vec<String>) -> String {
		let unicode: Vec<char> = vec!('\u{1F1E6}', '\u{1F1E7}', '\u{1F1E8}', '\u{1F1E9}', '\u{1F1EA}', '\u{1F1EB}', '\u{1F1EC}', '\u{1F1ED}', '\u{1F1EE}', '\u{1F1EF}', '\u{1F1F0}', '\u{1F1F1}', '\u{1F1F2}', '\u{1F1F3}', '\u{1F1F4}', '\u{1F1F5}', '\u{1F1F6}', '\u{1F1F7}', '\u{1F1F8}', '\u{1F1F9}', '\u{1F1FA}', '\u{1F1FB}', '\u{1F1FC}', '\u{1F1FD}', '\u{1F1FE}', '\u{1F1FF}');
		let alphabet: Vec<char> = vec!('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
		let ref word = args[0]; // ignore rest of args
		let last_message = d.get_messages(msg.channel_id, discord::GetMessages::MostRecent, Some(1)).expect("Last message not found!?");
		for ch in word.chars() {
			match alphabet.iter().position(|&x| x == ch) {
				Some(pos) => d.add_reaction(msg.channel_id, last_message[0].id, ReactionEmoji::Unicode(unicode[pos].to_string())).expect("Failed to add reaction."),
				None => () // skip
			}
		}
		String::new()
	}
}

pub struct TextTransformPlugin;
impl MPlugin for TextTransformPlugin {
    fn id(&self) -> String { String::from("trans") }
    fn execute(&self, d: &Discord, msg: &Message, args: Vec<String>) -> String {
    	let form = &*args[0];
    	let text = args[1..].iter().fold(String::new(), |acc, s| acc + " " + s);
    	match form {
    	    "b" => return text.to_math_bold(),
    	    "i" => return text.to_math_italic(),
    	    "bi" => return text.to_math_bold_italic(),
    	    "ss" => return text.to_math_sans_serif(),
    	    "ssb" => return text.to_math_sans_serif_bold(),
    	    "ssi" => return text.to_math_sans_serif_italic(),
    	    "ssbi" => return text.to_math_sans_serif_bold_italic(),
    	    "s" => return text.to_math_script(),
    	    "bs" => return text.to_math_bold_script(),
    	    "f" => return text.to_math_fraktur(),
    	    "m" => return text.to_math_monospace(),
    	    "ds" => return text.to_math_double_struck(),
    	    _ => return format!("Text Transform {} not found.", form)
    	}
    }
}