use regex::Regex;

use crate::{
    utils::url::UrlMisc,
    ui::macros_alerts::MacrosAlerts,
    regexp::regex_macros::MacrosRegExp,
};

pub struct MacroHandler;

impl MacroHandler {

    pub fn remove_macros(input: &str) -> String {
        let re = Regex::new(MacrosRegExp::GET_MACROS).unwrap();
        re.replace_all(input, "").to_string()
    }

    pub fn handle_check_macro_line(line: &str, word: &str) -> bool {
        let get_macro = format!(
            "!{}", word.to_lowercase()
        );

        line.contains(&get_macro)
    }
  
    pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
        if !no_ignore && Self::handle_check_macro_line(line, "ignore") {
            MacrosAlerts::ignore(line);
            return Err("Line contains the '!ignore' directive.");
        }
    
        Ok(
            UrlMisc::extract_url(line)
        )
    }

}
