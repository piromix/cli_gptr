use anyhow::{anyhow, Result};
use spinners::{Spinner, Spinners, Stream};
use std::io;
use std::io::{BufRead, BufReader};

use crate::args::Args;
use crate::config::Config;
use crate::gpt::{Client, Message};
use crate::predefine::Predefines;

mod args;
mod config;
mod deepl;
mod file;
mod gpt;
mod predefine;

pub fn check_args() -> Result<Args> {
    let args = Args::get_args().unwrap();
    return Ok(args);
}

fn read(args: &Args) -> Result<String> {
    match args.prompt.as_str() {
        "-" => {
            let mut text: String = String::new();
            let buf = Box::new(BufReader::new(io::stdin()));
            for (_, line_result) in buf.lines().enumerate() {
                text.push_str(&line_result?);
                text.push_str("\n");
            }
            Ok(text)
        }
        _ => Ok(args.prompt.clone()),
    }
}

async fn get_prompts(args: &Args, config: &Config) -> Result<Vec<Message>> {
    let prompt = read(&args)?;

    let mut messages: Vec<Message> = vec![];
    let predefines = Predefines::new();

    if args.predefine.is_some() {
        match predefines.get_predefine(args.predefine.as_ref().unwrap()) {
            Some(p) => {
                if p.translate {
                    let translate = deepl::Deepl::new(config.deepl_api_key.clone())
                        .translate(
                            &p.content,
                            Some(config.translate_source.clone()),
                            config.translate_target.clone(),
                        )
                        .await
                        .unwrap();
                    println!("translated user prompt: {}", translate);
                    messages.push(Message {
                        content: translate,
                        role: "system".to_string(),
                    });
                } else {
                    println!("user prompt: {}", p.content);
                    messages.push(Message {
                        content: p.content.clone(),
                        role: "system".to_string(),
                    })
                }
            }
            None => {}
        }
    }

    if args.translate {
        let translate = deepl::Deepl::new(config.deepl_api_key.clone())
            .translate(
                &prompt,
                Some(config.translate_source.clone()),
                config.translate_target.clone(),
            )
            .await
            .unwrap();
        println!("translated prompt: {}", translate);
        messages.push(Message {
            content: translate,
            role: "user".to_string(),
        });
    } else {
        messages.push(Message {
            content: prompt.clone(),
            role: "user".to_string(),
        });
    }

    return Ok(messages);
}

async fn translate_answer_if_need(args: &Args, config: &Config, answer: String) -> String {
    return if args.translate {
        println!("chatGPT response: {}", answer);
        deepl::Deepl::new(config.deepl_api_key.clone())
            .translate(
                &answer,
                Some(config.translate_target.clone()),
                config.translate_source.clone(),
            )
            .await
            .unwrap()
    } else {
        answer
    };
}

fn is_exist_config() -> bool {
    let config = file::read_config("gptr.json".to_string());
    if !config.is_some() {
        return std::env::var("OPENAI_API_KEY").is_ok();
    }
    return true;
}

pub async fn run(args: Args) -> Result<()> {
    if !is_exist_config() {
        return Err(anyhow!("Please create a gptr.json file."));
    }

    let config = Config::new();

    if !config.deepl_api_key.is_some() && args.translate {
        return Err(anyhow!(
            "Please set deepl_api_key in gptr.json file when using translate option."
        ));
    }

    let prompts = get_prompts(&args, &config).await;

    let mut sp = Spinner::with_stream(
        Spinners::Dots9,
        "Waiting for chatGPT response...".to_string(),
        Stream::Stderr,
    );

    let res = Client::new()
        .async_query(prompts.unwrap(), &config.openai_api_key)
        .await?;

    sp.stop();
    println!("\n\n");

    let response = translate_answer_if_need(&args, &config, res).await;

    println!("{}", response);

    Ok(())
}
