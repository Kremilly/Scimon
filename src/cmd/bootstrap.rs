use reqwest;

use std::{
    fs::File,
    error::Error,
    io::{BufReader, BufRead}
};

use crate::{
    ui::ui_base::PaimonUI,
    monlib::api_get_list::ApiGetList,

    cmd::{
        syntax::Lexico,
        download::Download,
    },

    utils::{
        misc::Misc,
        url::UrlMisc,
        file::FileMisc,
        validation::Validate
    }
};

pub struct Paimon;

impl Paimon {

    async fn read_lines<R>(reader: R, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> where R: BufRead {
        let mut path = String::new();

        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
    
            if !Lexico::handle_check_macro_line(&trimmed_line, "open_link") {
                if path.is_empty() {
                    path = Lexico::handle_get_path(trimmed_line);
                    let _ = FileMisc::new_path(&path);
                }
    
                let url = if !trimmed_line.contains("arxiv.org") {
                    trimmed_line.to_owned()
                } else {
                    trimmed_line.replace("/abs/", "/pdf/")
                };
    
                Download::download_file(
                    &url,
                    &path,
                    no_ignore,
                    no_comments,
                    kindle.clone()
                ).await?;
            } else {
                UrlMisc::open_url(trimmed_line);
            }
        }
    
        Ok(())
    }    

    pub async fn read_local_file(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let _ = Validate::validate_file(run).map_err(|e| {
            eprintln!("{}", e);
        });
        
        let file = File::open(run)?;
        let reader = BufReader::new(file);

        Self::read_lines(reader, no_ignore, no_comments, kindle).await?;
        Ok(())
    }
    
    pub async fn read_remote_file(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let _ = Validate::validate_file_type(run, ".txt").map_err(|e| {
            eprintln!("{}", e);
        });
    
        let response = reqwest::get(run).await?;
        let bytes = response.bytes().await?;
        let reader: BufReader<&[u8]> = BufReader::new(&bytes[..]);

        Self::read_lines(reader, no_ignore, no_comments, kindle).await?;
        Ok(())
    }
    
    pub async fn run(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            Self::read_local_file(
                run, no_ignore, no_comments, kindle
            ).await?;
        } else {
            Self::read_remote_file(
                run, no_ignore, no_comments, kindle
            ).await?;
        }

        Ok(())
    }

    pub async fn basic(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) {
        if !run.is_empty() {
            PaimonUI::header();
            
            if !Misc::check_is_user(run) {
                let _ = Paimon::run(
                    run, no_ignore, no_comments, kindle
                ).await;
            } else {
                let _ = ApiGetList::get(
                    run, no_ignore, no_comments, kindle
                ).await;
            }
        }
    }

}
