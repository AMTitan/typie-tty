use std::process;
use clap::{Arg, App};
use std::io::{Write, Read};
use std::net::TcpListener;
use rand::seq::SliceRandom;
use rand::Rng;
use std::process::Command;
use termion::color;
use terminal_size::{Width, Height, terminal_size};
use ncurses::*;

fn main() {
    let matches = App::new("typie-tty")
        .version("1.0")
        .about("typing test in the terminal")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .help("host a server")
            .takes_value(false))
        .arg(Arg::with_name("join")
            .short("j")
            .long("join")
            .help("join a server")
            .takes_value(true))
        .get_matches();
    if matches.is_present("host") {
        let words = vec!["a", "ability", "able", "about", "above", "accept", "according", "account", "across", "act", "action", "activity", "actually", "add", "address", "administration", "admit", "adult", "affect", "after", "again", "against", "age", "agency", "agent", "ago", "agree", "agreement", "ahead", "air", "all", "allow", "almost", "alone", "along", "already", "also", "although", "always", "American", "among", "amount", "analysis", "and", "animal", "another", "answer", "any", "anyone", "anything", "appear", "apply", "approach", "area", "argue", "arm", "around", "arrive", "art", "article", "artist", "as", "ask", "assume", "at", "attack", "attention", "attorney", "audience", "author", "authority", "available", "avoid", "away", "baby", "back", "bad", "bag", "ball", "bank", "bar", "base", "be", "beat", "beautiful", "because", "become", "bed", "before", "begin", "behavior", "behind", "believe", "benefit", "best", "better", "between", "beyond", "big", "bill", "billion", "bit", "black", "blood", "blue", "board", "body", "book", "born", "both", "box", "boy", "break", "bring", "brother", "budget", "build", "building", "business", "but", "buy", "by", "call", "camera", "campaign", "can", "cancer", "candidate", "capital", "car", "card", "care", "career", "carry", "case", "catch", "cause", "cell", "center", "central", "century", "certain", "certainly", "chair", "challenge", "chance", "change", "character", "charge", "check", "child", "choice", "choose", "church", "citizen", "city", "civil", "claim", "class", "clear", "clearly", "close", "coach", "cold", "collection", "college", "color", "come", "commercial", "common", "community", "company", "compare", "computer", "concern", "condition", "conference", "Congress", "consider", "consumer", "contain", "continue", "control", "cost", "could", "country", "couple", "course", "court", "cover", "create", "crime", "cultural", "culture", "cup", "current", "customer", "cut", "dark", "data", "daughter", "day", "dead", "deal", "death", "debate", "decade", "decide", "decision", "deep", "defense", "degree", "Democrat", "democratic", "describe", "design", "despite", "detail", "determine", "develop", "development", "die", "difference", "different", "difficult", "dinner", "direction", "director", "discover", "discuss", "discussion", "disease", "do", "doctor", "dog", "door", "down", "draw", "dream", "drive", "drop", "drug", "during", "each", "early", "east", "easy", "eat", "economic", "economy", "edge", "education", "effect", "effort", "eight", "either", "election", "else", "employee", "end", "energy", "enjoy", "enough", "enter", "entire", "environment", "environmental", "especially", "establish", "even", "evening", "event", "ever", "every", "everybody", "everyone", "everything", "evidence", "exactly", "example", "executive", "exist", "expect", "experience", "expert", "explain", "eye", "face", "fact", "factor", "fail", "fall", "family", "far", "fast", "father", "fear", "federal", "feel", "feeling", "few", "field", "fight", "figure", "fill", "film", "final", "finally", "financial", "find", "fine", "finger", "finish", "fire", "firm", "first", "fish", "five", "floor", "fly", "focus", "follow", "food", "foot", "for", "force", "foreign", "forget", "form", "former", "forward", "four", "free", "friend", "from", "front", "full", "fund", "future", "game", "garden", "gas", "general", "generation", "get", "girl", "give", "glass", "go", "goal", "good", "government", "great", "green", "ground", "group", "grow", "growth", "guess", "gun", "guy", "hair", "half", "hand", "hang", "happen", "happy", "hard", "have", "he", "head", "health", "hear", "heart", "heat", "heavy", "help", "her", "here", "herself", "high", "him", "himself", "his", "history", "hit", "hold", "home", "hope", "hospital", "hot", "hotel", "hour", "house", "how", "however", "huge", "human", "hundred", "husband", "I", "idea", "identify", "if", "image", "imagine", "impact", "important", "improve", "in", "include", "including", "increase", "indeed", "indicate", "individual", "industry", "information", "inside", "instead", "institution", "interest", "interesting", "international", "interview", "into", "investment", "involve", "issue", "it", "item", "its", "itself", "job", "join", "just", "keep", "key", "kid", "kill", "kind", "kitchen", "know", "knowledge", "land", "language", "large", "last", "late", "later", "laugh", "law", "lawyer", "lay", "lead", "leader", "learn", "least", "leave", "left", "leg", "legal", "less", "let", "letter", "level", "lie", "life", "light", "like", "likely", "line", "list", "listen", "little", "live", "local", "long", "look", "lose", "loss", "lot", "love", "low", "machine", "magazine", "main", "maintain", "major", "majority", "make", "man", "manage", "management", "manager", "many", "market", "marriage", "material", "matter", "may", "maybe", "me", "mean", "measure", "media", "medical", "meet", "meeting", "member", "memory", "mention", "message", "method", "middle", "might", "military", "million", "mind", "minute", "miss", "mission", "model", "modern", "moment", "money", "month", "more", "morning", "most", "mother", "mouth", "move", "movement", "movie", "Mr", "Mrs", "much", "music", "must", "my", "myself", "name", "nation", "national", "natural", "nature", "near", "nearly", "necessary", "need", "network", "never", "new", "news", "newspaper", "next", "nice", "night", "no", "none", "nor", "north", "not", "note", "nothing", "notice", "now", "n't", "number", "occur", "of", "off", "offer", "office", "officer", "official", "often", "oh", "oil", "ok", "old", "on", "once", "one", "only", "onto", "open", "operation", "opportunity", "option", "or", "order", "organization", "other", "others", "our", "out", "outside", "over", "own", "owner", "page", "pain", "painting", "paper", "parent", "part", "participant", "particular", "particularly", "partner", "party", "pass", "past", "patient", "pattern", "pay", "peace", "people", "per", "perform", "performance", "perhaps", "period", "person", "personal", "phone", "physical", "pick", "picture", "piece", "place", "plan", "plant", "play", "player", "PM", "point", "police", "policy", "political", "politics", "poor", "popular", "population", "position", "positive", "possible", "power", "practice", "prepare", "present", "president", "pressure", "pretty", "prevent", "price", "private", "probably", "problem", "process", "produce", "product", "production", "professional", "professor", "program", "project", "property", "protect", "prove", "provide", "public", "pull", "purpose", "push", "put", "quality", "question", "quickly", "quite", "race", "radio", "raise", "range", "rate", "rather", "reach", "read", "ready", "real", "reality", "realize", "really", "reason", "receive", "recent", "recently", "recognize", "record", "red", "reduce", "reflect", "region", "relate", "relationship", "religious", "remain", "remember", "remove", "report", "represent", "Republican", "require", "research", "resource", "respond", "response", "responsibility", "rest", "result", "return", "reveal", "rich", "right", "rise", "risk", "road", "rock", "role", "room", "rule", "run", "safe", "same", "save", "say", "scene", "school", "science", "scientist", "score", "sea", "season", "seat", "second", "section", "security", "see", "seek", "seem", "sell", "send", "senior", "sense", "series", "serious", "serve", "service", "set", "seven", "several", "sex", "sexual", "shake", "share", "she", "shoot", "short", "shot", "should", "shoulder", "show", "side", "sign", "significant", "similar", "simple", "simply", "since", "sing", "single", "sister", "sit", "site", "situation", "six", "size", "skill", "skin", "small", "smile", "so", "social", "society", "soldier", "some", "somebody", "someone", "something", "sometimes", "son", "song", "soon", "sort", "sound", "source", "south", "southern", "space", "speak", "special", "specific", "speech", "spend", "sport", "spring", "staff", "stage", "stand", "standard", "star", "start", "state", "statement", "station", "stay", "step", "still", "stock", "stop", "store", "story", "strategy", "street", "strong", "structure", "student", "study", "stuff", "style", "subject", "success", "successful", "such", "suddenly", "suffer", "suggest", "summer", "support", "sure", "surface", "system", "table", "take", "talk", "task", "tax", "teach", "teacher", "team", "technology", "television", "tell", "ten", "tend", "term", "test", "than", "thank", "that", "the", "their", "them", "themselves", "then", "theory", "there", "these", "they", "thing", "think", "third", "this", "those", "though", "thought", "thousand", "threat", "three", "through", "throughout", "throw", "thus", "time", "to", "today", "together", "tonight", "too", "top", "total", "tough", "toward", "town", "trade", "traditional", "training", "travel", "treat", "treatment", "tree", "trial", "trip", "trouble", "true", "truth", "try", "turn", "TV", "two", "type", "under", "understand", "unit", "until", "up", "upon", "us", "use", "usually", "value", "various", "very", "victim", "view", "violence", "visit", "voice", "vote", "wait", "walk", "wall", "want", "war", "watch", "water", "way", "we", "weapon", "wear", "week", "weight", "well", "west", "western", "what", "whatever", "when", "where", "whether", "which", "while", "white", "who", "whole", "whom", "whose", "why", "wide", "wife", "will", "win", "wind", "window", "wish", "with", "within", "without", "woman", "wonder", "word", "work", "worker", "world", "worry", "would", "write", "writer", "wrong", "yard", "yeah", "year", "yes", "yet", "you", "young", "your", "yourself"];
        let mut paragraph = "".to_string();
        for _ in 0..(rand::thread_rng().gen_range(100..200)) {
            paragraph.push_str(&[words.choose(&mut rand::thread_rng()).unwrap().to_string().as_str() , " "].join(""));
        }
        paragraph = paragraph.trim().to_string();
        let mut ips:Vec<String> = Vec::new();
        println!("public ip address: {}", ip());
        let listener = TcpListener::bind("0.0.0.0:5803").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let mut get_next = false;
            let mut data = "".to_string();
            let mut length = 0;
            for i in String::from_utf8_lossy(&buffer[..]).split("\n") {
                if get_next == true && !i.trim().is_empty() {
                    data.push_str(i.trim());
                }
                if i.starts_with("Content-Length: ") {
                    length = i.replace("Content-Length: ", "").trim().parse().unwrap();
                }
                if i.trim() == "" {
                    get_next = true;
                }
            }
            data = (&data[..length]).to_string();
            println!("{}", data);
            if data.starts_with("join") {
                ips.push(data[5..data.len()].to_string());
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    paragraph.len(),
                    paragraph
                );
                stream.write(response.as_bytes()).unwrap();
            }
            else {
                stream.write("HTTP/1.1 404\nContent-Length: 0".as_bytes()).unwrap();
            }
            stream.flush().unwrap();
        }
    } 
    else if matches.is_present("join") {
        let returns = Command::new("curl")
            .args(["-X", "POST", &[matches.value_of("join").unwrap() , ":5803"].join(""), "-d", &["join ".to_string(), ip()].join("")])
            .output()
            .expect("ls command failed to start");
        let paragraph = String::from_utf8(returns.stdout).unwrap();
        let mut wrote = "".to_string();
        while wrote.clone().len() <= paragraph.clone().len() {
            initscr();
            refresh();
            println!("{}", print_paragraph(paragraph.clone(), wrote.clone()));
            wrote.push((getch() as u8) as char);
        }
    }
    else {
        println!("You have to join or host and game type --help to see how to do so");
    }
}


#[tokio::main]
async fn ip() -> String {
    // Attempt to get an IP address and print it.
    if let Some(ip) = public_ip::addr().await {
        return ip.to_string();
    } else {
        println!("couldn't get an IP address\n");
        process::exit(520);
    }
}

fn print_paragraph(paragraph:String, typed:String) -> String {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        let mut returns = "".to_string();
        returns.push_str(format!("{}╭{}╮", color::Fg(color::Reset), "-".repeat((w-2).into())).as_str());
        for x in 0..paragraph.len()/(((w-2) as f32).ceil()) as usize {
            returns.push_str(format!("{}|", color::Fg(color::Reset)).as_str());
            for y in (x*((w-2) as usize))..((x+1)*((w-2) as usize)) {
                match paragraph.chars().nth(y) {
                    Some(char) => {
                        if typed.len() > y {
                            if typed.chars().nth(y).unwrap() == char {
                                returns.push_str(format!("{}{}", color::Fg(color::LightBlack), char).as_str());
                            }
                            else {
                                returns.push_str(format!("{}{}", color::Fg(color::Red), char).as_str());
                            }
                        }
                        else {
                            returns.push_str(format!("{}{}", color::Fg(color::Reset), char).as_str());
                        }
                    }
                    None => {
                        returns.push_str(format!("{} ", color::Fg(color::Reset)).as_str());
                    }
                }
            }
            returns.push_str(format!("{}|", color::Fg(color::Reset)).as_str());
        }
        returns.push_str(format!("{}╰{}╯", color::Fg(color::Reset), "-".repeat((w-2).into())).as_str());
        returns
    } else {
        println!("Unable to get terminal size");
        "".to_string()
    }
}