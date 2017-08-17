
use eval::{eval, to_value};
use std::str::FromStr;
use std::error::Error;
use math_text_transform::MathTextTransform;
use *;
use discord::model::ReactionEmoji; 
use rusqlite::Connection;
use std::path::Path;


#[derive(Debug, Clone)]
struct MemoryChunk {
    from: String,
    into: String,
}

impl MemoryChunk {
    fn new(from: String, into: String) -> MemoryChunk {
        MemoryChunk {
            from: from,
            into: into
        }
    }
}

fn insert_memory(conn: &Connection, chunk: MemoryChunk) {
    conn.execute("INSERT INTO memory (msg_from, msg_into) 
                  VALUES (?1, ?2)", &[&chunk.from, &chunk.into]).unwrap();
}

fn whole_memory(conn: &Connection) -> Vec<MemoryChunk> {
    let mut stmt = conn.prepare("SELECT msg_from, msg_into FROM memory").unwrap();
    let chunk_iter = stmt.query_map(&[], |row|
        MemoryChunk {
            from: row.get(0),
            into: row.get(1)
        }
    ).unwrap();
    let mut chunks: Vec<MemoryChunk> = Vec::new();
    for chunk_res in chunk_iter {
        chunks.push( chunk_res.unwrap() );
    }
    chunks
}

fn fetch_memory(conn: &Connection, from: String) -> Option<MemoryChunk> {
    let chunks = whole_memory(conn);
    chunks.iter().find(|&x| x.from == from).cloned()
}

fn delete_memory(conn: &Connection, from: String) {
    conn.execute("DELETE FROM memory WHERE msg_from = (?1)", &[&from]).unwrap();
}

fn create_table(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS memory (
        msg_from        TEXT NOT NULL PRIMARY KEY,
        msg_into        TEXT NOT NULL
        )", &[]).unwrap();
}

/* Save */
pub struct RememberPlugin;
impl MPlugin for RememberPlugin {
    fn id(&self) -> Vec<&str> { vec!("rem", "sa", "@") }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        let from = data.arguments[0].clone();
        let into = data.arguments.into_iter().skip(1).collect::<Vec<String>>().join(" ");
        create_table(&conn);
        insert_memory(&conn, MemoryChunk::new(from, into));
        String::new()
    }
}

pub struct ForgetPlugin;
impl MPlugin for ForgetPlugin {
    fn id(&self) -> Vec<&str> { vec!("#") }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        delete_memory(&conn, data.arguments[0].clone());
        String::new()
    }
}

pub struct OmnipotencePlugin;
impl MPlugin for OmnipotencePlugin {
    fn id(&self) -> Vec<&str> { vec!("!!") }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        let memory = whole_memory(&conn);
        let mut data = String::new();
        for chunk in memory {
            data.push_str(&*format!("{} => {}\n", chunk.from, chunk.into));
        }
        format!("```Memory:\n{}```", data)
    }
}

/* Load */
pub struct RecallPlugin;
impl MPlugin for RecallPlugin {
    fn id(&self) -> Vec<&str> { vec!("rec", "!") }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        let name = data.arguments[0].clone();
        create_table(&conn);
        let memory = fetch_memory(&conn, name.clone());
        if memory.is_some() {
            return memory.unwrap().into;
        } else {
            return format!("Memory \"{}\" does not exist.", name);
        }
    }
}

pub struct PurgePlugin;
impl MPlugin for PurgePlugin {
   fn id(&self) -> Vec<&str> { vec!("purge", "clear") }
   fn execute(&self, data: PluginData) -> String {
      if data.arguments.len() == 1 {
         let ref num_up = data.arguments[0];
         let num: u64 = FromStr::from_str(&*num_up).expect("Failed to parse purge count.");
         let mut deleted: u64 = 0;
         let mut attemps: u64 = 1;
         while deleted < num {
             let ref last_msg = data.discord.get_messages(data.message.channel_id, discord::GetMessages::MostRecent, Some(1)).expect("Failed to get the last message.")[0];
             if last_msg.author.id == data.message.author.id {
                data.discord.delete_message(last_msg.channel_id, last_msg.id).ok();
                deleted += 1;
             } else {
                if attemps > 200 {
                    break;
                } else {
                    let messages = data.discord.get_messages(data.message.channel_id, discord::GetMessages::MostRecent, Some(attemps)).expect("Failed to get recent messages.");
                    for message in messages {
                        if message.author.id == data.message.author.id {
                            data.discord.delete_message(message.channel_id, message.id).ok();
                            deleted += 1;
                        }
                    }
                    attemps += 1;
                }
             }
         }
         /* 
         while (msgs.len() as u64) < num {
            let mut others = data.discord.get_messages(data.message.channel_id, discord::GetMessages::MostRecent, Some(index_buffer))
               .unwrap_or(Vec::new());
            println!("adding {:?}", others);
            msgs.append( &mut others );
         }
         for msg in msgs.iter().filter(|x| x.author.id == data.message.author.id) {
            data.discord.delete_message(msg.channel_id, msg.id).ok();
         }*/
      }

      String::new()
   }
}

pub struct ShillPlugin;
impl MPlugin for ShillPlugin {
    fn id(&self) -> Vec<&str> { vec!("shill", "box") }
    fn execute(&self, data: PluginData) -> String {
        if data.arguments[0].len() > 0 {
            let ref text = data.arguments.join(" ");
            let mut base = text.chars().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
            for rest in text.chars().skip(1) {
                base.push_str(&*format!("\n{}", rest));
            }
            return format!("```{}```", base);
        }
        String::from("Blank Message")   
    }
}

pub struct SettingsPlugin;
impl MPlugin for SettingsPlugin {

   fn id(&self) -> Vec<&str> { vec!("settings", "edit") }
   fn execute(&self, data: PluginData) -> String {
      let ref args = data.arguments;
      let ref setting = *args[0].to_lowercase();
      let value = if args.len() > 1 { Some(args[1].clone()) } else { None };
      match setting {
         "mark" | "embed" => {
            let mut z = None;
            if value.is_some() {
               let n_val = value.unwrap();
               let string_res: Result<bool, _> = FromStr::from_str(&*n_val);
               if string_res.is_ok() {
                  z = data.settings.set(setting, string_res.unwrap(), false);
               }
            } else {
               z = data.settings.set(setting, false, true)
            }
            if z.is_some() {
               format!("{} := `{:?}`", setting, z.unwrap())
            } else {
               String::new()
            }
         },
         _ => String::new()
      }
   }
}

pub struct EvalPlugin;
impl MPlugin for EvalPlugin {
    fn id(&self) -> Vec<&str> { vec!(";", "e", "eval") }
    fn execute(&self, data: PluginData) -> String {
        let text = data.arguments.join(" ");
        match eval(&*text) {
            Ok(val) => format!("{} = {}", text, to_value(val)),
            Err(why) => String::from(why.description()),
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
    			b.color(0xFFFFFF)
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
    	let text = args[1..].join(" ");
    	match &*args[0] {
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

pub struct AboutPlugin;
impl MPlugin for AboutPlugin {
    fn id(&self) -> Vec<&str> { vec!("about", "bot") }
    fn execute(&self, data: PluginData) -> String {
        let ref d = data.discord;
        let ref msg = data.message;
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        d.send_embed(msg.channel_id, "", |b| { 
            b.color(0xFF3333)
            .url("https://github.com/hvze/misaki")
            .title("Misaki (Modular Selfbot)")
            .thumbnail("https://ill.fi/ncdt.png")
            .fields(|f| {
                f
                .field("Memory Chunks", &*whole_memory(&conn).len().to_string(), true)    
                .field("Modules", "12" /* lol */, true)
            })
        }).expect("Failed to send embed.");
        String::new()
    }
}