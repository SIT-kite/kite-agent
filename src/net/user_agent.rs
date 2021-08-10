use rand::{seq::SliceRandom, thread_rng};
use std::fs::File;
use std::io::Read;

/// User-Agent list filename.
const UA_FILE: &str = "user-agents.txt";
/// Default User-Agent string could be used when UA_FILE can not be read.
const DEFAULT_UA_STRING: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:75.0) Gecko/20100101 Firefox/75.0";

lazy_static! {
    pub static ref UA_STRINGS: Vec<String> = load_ua_strings(UA_FILE);
}

/// Load browser user agent string from text file.
fn load_ua_strings(filename: &str) -> Vec<String> {
    let f: std::io::Result<File> = File::open(filename);
    if let Ok(mut f) = f {
        let mut buffer = String::new();
        if f.read_to_string(&mut buffer).is_ok() {
            let strings: Vec<String> = buffer.lines().map(|line| line.to_string()).collect();

            return strings;
        }
    }
    vec![DEFAULT_UA_STRING.to_string()]
}

/// Choose a random ua string from UA_STRINGS.
pub fn get_random_ua_string() -> &'static String {
    let mut rng = thread_rng();

    return UA_STRINGS[..].choose(&mut rng).unwrap();
}
