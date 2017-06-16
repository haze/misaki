extern crate discord;		
extern crate math_text_transform;

use std::str::FromStr;
use math_text_transform::MathTextTransform;
use *;
use discord::model::ReactionEmoji; 


pub struct SettingsPlugin;
impl SettingsPlugin {
   fn get_setting_refrence<'a>(&self, s_name: &str, settings: &'a MisakiSettings) -> Option<&'a bool> {
      return match &*String::from(s_name).to_lowercase() {
          "mark" => Some(&settings.should_mark),
          "embed" => Some(&settings.embed_mode),
          _ => None
      }
   }
}
impl MPlugin for SettingsPlugin {

   fn id(&self) -> Vec<&str> { vec!("settings", "edit") }
   fn execute(&self, data: PluginData) -> String {
      let ref args = data.arguments;
      let ref setting = *args[0].to_lowercase();
      let value = if args.len() > 1 { Some(args[1].clone()) } else { None };
      match setting {
         // mark messages with |=>
         "mark" | "embed" => {
            let mut r = self.get_setting_refrence(setting,data.settings).expect("Found no variable setting with name.");
            let old = r.clone();
            match value {
               Some(n_val) => { 
                  let b: Result<bool, _> = FromStr::from_str(&*n_val);
                  match b {
                     Ok(b_fin) => r = &b_fin,
                     Err(why) => return format!("{:?}", why),
                  };
               },
               None => r = &!old,
            };
            format!("{} := `{:?}`", setting, r)
         },
         _ => String::new()
      }
   }
}

pub struct UserInfoPlugin;
impl MPlugin for UserInfoPlugin {
    fn id(&self) -> Vec<&str> { vec!("dox", "whois", "usr") }
    fn execute(&self, data: PluginData) -> String {
        let ref msg = data.message;
        let ref d = data.discord;
    	
        if msg.mentions.len() > 0 {
    		let ref mem = msg.mentions[0];
    		let avatar_url = d.get_user_avatar_url(mem.id, mem.avatar.as_ref().unwrap());
    		d.send_embed(msg.channel_id, "", |b| { 
    			b.color(131313)
    			.title(&*mem.name)
    			.thumbnail(&avatar_url)
                .fields(|f| {
                    f
                    .field("Discriminator", &*mem.discriminator.to_string(), true)
                    .field("Bot?", &*mem.bot.to_string(), true)
                    .field("Id", &*mem.id.to_string(), true)
                })
    		}).expect("Failed to send embed.");
    	}
    	String::new()
    }
}

pub struct ReactPlugin;
impl MPlugin for ReactPlugin {
	fn id(&self) -> Vec<&str> { vec!("react", "re") }
	fn execute(&self, data: PluginData) -> String {
		let ref args = data.arguments;
      let ref msg = data.message;
      let ref d = data.discord;

      let unicode: Vec<char> = vec!('\u{1F1E6}', '\u{1F1E7}', '\u{1F1E8}', '\u{1F1E9}', '\u{1F1EA}', '\u{1F1EB}', '\u{1F1EC}', '\u{1F1ED}', '\u{1F1EE}', '\u{1F1EF}', '\u{1F1F0}', '\u{1F1F1}', '\u{1F1F2}', '\u{1F1F3}', '\u{1F1F4}', '\u{1F1F5}', '\u{1F1F6}', '\u{1F1F7}', '\u{1F1F8}', '\u{1F1F9}', '\u{1F1FA}', '\u{1F1FB}', '\u{1F1FC}', '\u{1F1FD}', '\u{1F1FE}', '\u{1F1FF}');
		let alphabet: Vec<char> = vec!('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
		let ref word = args[0];
		let last_message = d.get_messages(msg.channel_id, discord::GetMessages::MostRecent, Some(1)).expect("Last message not found!?");
		for ch in word.chars() {
                            // bullshit .occurance(z)
            /* if word.chars().filter(|x| x == ch).count() > 1 {
                    do something if we have more than one? find alternatives in a map?
            } */
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
    fn id(&self) -> Vec<&str> { vec!("transf", "mt") }
    fn execute(&self, data: PluginData) -> String {
    	let ref args = data.arguments;
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
    	    _ => return String::new()
    	}
    }
}