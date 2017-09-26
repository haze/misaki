use std::str;
use eval::{eval, to_value};
use std::str::FromStr;
use std::error::Error;
use math_text_transform::MathTextTransform;
use misaki_api::misaki::{MPlugin, PluginData};
use std::collections::HashMap;
use std::fs::File;
use discord;
use discord::Discord;
use discord::model::ChannelId;
use rusqlite::Connection;
use std::path::Path;
use rand::Rng;
use rand;
use std::io::{Cursor, Read};
use curl::easy::{Easy, Form};

#[derive(Debug, Clone)]
struct MemoryChunk {
    from: String,
    into: String,
}

impl MemoryChunk {
    fn new(from: String, into: String) -> MemoryChunk {
        MemoryChunk {
            from: from,
            into: into,
        }
    }
}

fn insert_memory(conn: &Connection, chunk: MemoryChunk) {
    conn.execute(
        "INSERT INTO memory (msg_from, msg_into) 
                  VALUES (?1, ?2)",
        &[&chunk.from, &chunk.into],
    ).unwrap();
}

fn whole_memory(conn: &Connection) -> Vec<MemoryChunk> {
    let mut stmt = conn.prepare("SELECT msg_from, msg_into FROM memory")
        .unwrap();
    let chunk_iter = stmt.query_map(&[], |row| {
        MemoryChunk {
            from: row.get(0),
            into: row.get(1),
        }
    }).unwrap();
    let mut chunks: Vec<MemoryChunk> = Vec::new();
    for chunk_res in chunk_iter {
        chunks.push(chunk_res.unwrap());
    }
    chunks
}

fn fetch_memory(conn: &Connection, from: String) -> Option<MemoryChunk> {
    let chunks = whole_memory(conn);
    chunks.iter().find(|&x| x.from == from).cloned()
}

fn delete_memory(conn: &Connection, from: String) {
    conn.execute("DELETE FROM memory WHERE msg_from = (?1)", &[&from])
        .unwrap();
}

fn create_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
        msg_from        TEXT NOT NULL PRIMARY KEY,
        msg_into        TEXT NOT NULL
        )",
        &[],
    ).unwrap();
}

/* Save */
pub struct RememberPlugin;
impl MPlugin for RememberPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["rem", "sa", "@"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        let from = data.arguments[0].clone();
        let into = data.arguments
            .into_iter()
            .skip(1)
            .collect::<Vec<String>>()
            .join(" ");
        create_table(&conn);
        insert_memory(&conn, MemoryChunk::new(from, into));
        String::new()
    }
}

pub struct ForgetPlugin;
impl MPlugin for ForgetPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["#"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        delete_memory(&conn, data.arguments[0].clone());
        String::new()
    }
}

pub struct OmnipotencePlugin;
impl MPlugin for OmnipotencePlugin {
    fn id(&self) -> Vec<&str> {
        vec!["!!"]
    }
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
    fn id(&self) -> Vec<&str> {
        vec!["rec", "!"]
    }
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

/* For Misconception */
#[derive(Default, Debug)]
struct MarkedString {
    string: String,
    upper_indicies: Vec<u32>,
}

fn load_words() -> HashMap<Vec<String>, String> {
    let mut map = HashMap::new();
    let mut words_str: String = String::new();
    let mut file = File::open("res/mistaken.txt").unwrap();
    file.read_to_string(&mut words_str);
    for excerpt in words_str.lines() {
        let mut data = excerpt.split("->");
        let left = data.nth(0).unwrap().to_lowercase();
        let right = data.nth(0).unwrap();
        let a_right = right
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        map.insert(a_right, left);
    }
    map
}

pub struct MisconceptionPlugin;
impl MPlugin for MisconceptionPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["misconcept", "uhm"]
    }
    fn execute(&self, data: PluginData) -> String {

        fn to_misconception(text: String) -> String {
            let misconceptions = load_words();
            let words = text.split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            words
                .iter()
                .map(|x| {
                    let mut key: Vec<String> = Vec::new();
                    for key in misconceptions.keys() {
                        if key.contains(&x.to_string()) {
                            return misconceptions.get(key).unwrap().clone();
                        }
                    }
                    x.clone()
                })
                .collect::<Vec<String>>()
                .join(" ")
        }

        let ref msg = data.message;
        let ref d = data.discord;

        let last_msg_if = data.discord.get_messages(
            data.message.channel_id,
            discord::GetMessages::MostRecent,
            Some(1),
        );

        if last_msg_if.is_ok() {
            let last_msg = &last_msg_if.unwrap()[0];
            let last_msg_content = last_msg.content.clone();
            if last_msg.author.id == msg.author.id {
                d.edit_message(
                    last_msg.channel_id,
                    last_msg.id,
                    &*to_misconception(last_msg_content),
                );
            } else {
                return to_misconception(last_msg_content);
            }
        }
        String::new()
    }
}


pub struct PurgePlugin;
impl MPlugin for PurgePlugin {
    fn id(&self) -> Vec<&str> {
        vec!["purge", "clear"]
    }
    fn execute(&self, data: PluginData) -> String {
        if data.arguments.len() == 1 {
            let ref num_up = data.arguments[0];
            let num = FromStr::from_str(&*num_up).unwrap();
            let mut deleted: u64 = 0;
            let mut attemps: u64 = 1;
            while deleted < num {
                let last_msg_if = data.discord.get_messages(
                    data.message.channel_id,
                    discord::GetMessages::MostRecent,
                    Some(1),
                );
                if last_msg_if.is_ok() {
                    let last_msgs = &last_msg_if.unwrap();
                    if last_msgs.len() > 0 {
                        let ref last_msg = last_msgs[0];
                        if last_msg.author.id == data.message.author.id {
                            data.discord
                                .delete_message(last_msg.channel_id, last_msg.id)
                                .ok();
                            deleted += 1;
                        } else {
                            if attemps > 100 {
                                // Can't get most recent messages more than 100 times..
                                break;
                            } else {
                                let messages = data.discord
                                    .get_messages(
                                        data.message.channel_id,
                                        discord::GetMessages::MostRecent,
                                        Some(attemps),
                                    )
                                    .expect("Failed to get recent messages.");
                                for message in messages {
                                    if message.author.id == data.message.author.id {
                                        data.discord
                                            .delete_message(message.channel_id, message.id)
                                            .ok();
                                        deleted += 1;
                                    }
                                }
                                attemps += 1;
                            }
                        }
                    }
                }
            }
        }

        String::new()
    }
}



pub struct ShillPlugin;
impl MPlugin for ShillPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["shill", "box"]
    }
    fn execute(&self, data: PluginData) -> String {
        if data.arguments[0].len() > 0 {
            let ref text = data.arguments.join(" ");
            let mut base = text.chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");
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
    fn id(&self) -> Vec<&str> {
        vec!["settings", "edit"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref args = data.arguments;
        let ref setting = *args[0].to_lowercase();
        let value = if args.len() > 1 {
            Some(args[1].clone())
        } else {
            None
        };

        let mut bool_call = None;
        let mut num_call = None;
        let mut str_call = None;
        if value.is_some() {
            if data.settings.is_bool(setting) {
                let n_val = value.unwrap();
                let string_res: Result<bool, _> = FromStr::from_str(&*n_val);
                bool_call = data.settings.set(setting, string_res.unwrap(), false);
            } else if data.settings.is_number(setting) {
                let n_val = value.unwrap();
                let string_res: Result<u32, _> = FromStr::from_str(&*n_val);
                num_call = data.settings.set_num(setting, string_res.unwrap());
            } else {
                let n_val = value.unwrap();
                str_call = data.settings.set_str(setting, String::from(n_val));
            }
        } else {
            if data.settings.is_bool(setting) {
                bool_call = data.settings.set(setting, false, true)
            } else {
                return format!("{} is not a flippable variable.", setting);
            }
        }
        if bool_call.is_some() {
            format!("{} = {:?}", setting, bool_call.unwrap())
        } else if num_call.is_some() {
            format!("{} = {:?}", setting, num_call.unwrap())
        } else if str_call.is_some() {
            format!("{} = {:?}", setting, str_call.unwrap())
        } else {
            String::new()
        }
    }
}

pub struct EvalPlugin;
impl MPlugin for EvalPlugin {
    fn id(&self) -> Vec<&str> {
        vec![";", "e", "eval"]
    }
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
    fn id(&self) -> Vec<&str> {
        vec!["dox", "whois", "usr"]
    }
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
                        f.field("Discriminator", &*mem.discriminator.to_string(), true)
                            .field("Bot?", &*mem.bot.to_string(), true)
                            .field("Id", &*mem.id.to_string(), true)
                    })
            }).expect("Failed to send embed.");
        }
        String::new()
    }
}

pub struct MockPlugin;
impl MPlugin for MockPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["mock", "mo"]
    }

    fn execute(&self, data: PluginData) -> String {
        fn mock(text: String) -> String {
            let mut rng = rand::thread_rng();
            text.chars()
                .map(|x| if rng.gen() {
                    x.to_uppercase().to_string()
                } else {
                    x.to_lowercase().to_string()
                })
                .collect::<String>()
        }

        let ref args = data.arguments;
        let ref msg = data.message;
        let ref d = data.discord;

        let last_msg_if = data.discord.get_messages(
            data.message.channel_id,
            discord::GetMessages::MostRecent,
            Some(1),
        );
        if last_msg_if.is_ok() {
            let last_msg = &last_msg_if.unwrap()[0];
            let last_msg_content = last_msg.content.clone();
            if last_msg.author.id == msg.author.id {
                d.edit_message(last_msg.channel_id, last_msg.id, &*mock(last_msg_content));
            } else {
                return mock(last_msg_content);
            }
        }
        String::new()
    }
}

pub struct TextTransformPlugin;
impl MPlugin for TextTransformPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["transf", "mt"]
    }
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
            _ => return String::new(),
        }
    }
}

pub struct AboutPlugin;
impl MPlugin for AboutPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["about", "bot"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref d = data.discord;
        let ref msg = data.message;
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        d.send_embed(msg.channel_id, "", |b| {
            b.color(0xFF3333)
                .url("https://github.com/hvze/misaki")
                .title("Misaki (Modular Selfbot)")
                .thumbnail("https://ill.fi/ncdt.png")
                .fields(|f| {
                    f.field(
                        "Memory Chunks",
                        &*whole_memory(&conn).len().to_string(),
                        true,
                    ).field("Modules", "12" /* lol */, true)
                })
        }).expect("Failed to send embed.");
        String::new()
    }
}


fn send_latex(link: String, discord: &Discord, chid: ChannelId) {
    let mut easy = Easy::new();
    easy.url(&link).unwrap();

    let mut result = Vec::new();
    {
        let mut transfer = easy.transfer();

        transfer.write_function(|data| {
            result.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }

    discord.send_file(chid, "", Cursor::new(result), "latex.png");
}

// curl 'http://quicklatex.com/latex3.f' -H 'Origin: http://quicklatex.com' -H 'Accept-Encoding: gzip, deflate' -H 'Accept-Language: en-US,en;q=0.9,fr;q=0.8' -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.18 Safari/537.36' -H 'Content-Type: application/x-www-form-urlencoded' -H 'Accept: */*' -H 'Referer: http://quicklatex.com/' -H 'X-Requested-With: XMLHttpRequest' -H 'Connection: keep-alive' -H 'DNT: 1' --data $htbu t lktlhl th eh the formua she needs is this one write here im just bllshit typing because i know i can witout looking at the screen.' --compressed
pub struct LatexPlugin;

impl MPlugin for LatexPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["tex", "latex", ".."]
    }
    fn execute(&self, data: PluginData) -> String {
        let form = data.arguments.join(" ");
        let mut handle = Easy::new();
        handle.url("http://quicklatex.com/latex3.f").unwrap();
        let formula = format!("formula={}&fcolor={}", form, data.settings.latex_color);
        let chid = data.message.channel_id;
        let mut xdat = formula + "&fsize=17px&mode=0&out=1&remhost=quicklatex.com&preamble=\\usepackage{amsmath}\n\\usepackage{amsfonts}\n\\usepackage{amssymb}&rnd=66.52322504562316";
        handle.post_field_size(xdat.len() as u64).unwrap();
        // let mut result = String::new();

        let mut result = Vec::new();
        {
            let mut transfer = handle.transfer();

            transfer.write_function(|data| {
                result.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            transfer.read_function(|buf| {
                Ok(xdat.as_bytes().read(buf).unwrap_or(0))
            }).unwrap();

            transfer.perform().unwrap();
        }
        let res_str = String::from_utf8(result).unwrap();
        let chars = res_str.chars().skip(3);
        let _link = chars.collect::<String>();
        let link = _link.split(" ").nth(0).unwrap();
        send_latex(link.to_string(), data.discord, chid);
        String::new()
    }
}
