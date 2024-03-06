mod settings;

use std::{borrow::Cow, io::Write};
use twitch_archiver::{
    convert::{self},
    Twitch, TWITCH_VOD_URL_REGEX,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn collect_opt_stringed_arg<'a>(
    args: &mut impl Iterator<Item = &'a str>,
    preserve_quotes: bool,
) -> Option<Cow<'a, str>> {
    let first = match args.next() {
        Some(x) => x,
        None => return None,
    };

    if first.chars().nth(0).unwrap() != '"' {
        return Some(Cow::Borrowed(first));
    }

    let mut strbuf = String::with_capacity(first.len() + 20);
    let quote_start_index = if preserve_quotes { 0 } else { 1 };
    strbuf.push_str(&first[quote_start_index..]);

    for arg in args {
        strbuf.push(' ');
        if arg.chars().last().unwrap() == '"' {
            let quote_end_index = if preserve_quotes {
                arg.len()
            } else {
                arg.len() - 1
            };
            strbuf.push_str(&arg[0..quote_end_index]);
            break;
        }
        strbuf.push_str(arg);
    }

    Some(Cow::Owned(strbuf))
}

fn download<'a>(args: impl Iterator<Item = &'a str>) -> Result<()> {
    let mut args = args.peekable();

    let mut auth = settings::get()?.auth_token;
    let mut client_id = settings::get()?.client_id;
    let mut input_args = None;
    let mut output_args = None;
    let mut url = "";
    let mut out_path = None;

    loop {
        let arg = args.next();
        match arg {
            Some("--auth") => {
                let arg = collect_opt_stringed_arg(&mut args, false);
                auth = arg.map(|x| x.into_owned());
            }
            Some("--client-id") => {
                let arg = collect_opt_stringed_arg(&mut args, false);
                client_id = arg.map(|x| x.into_owned());
            }
            Some("--input-args") => {
                let arg = collect_opt_stringed_arg(&mut args, false);
                input_args = arg.map(|x| x.into_owned());
            }
            Some("--output-args") => {
                let arg = collect_opt_stringed_arg(&mut args, false);
                output_args = arg.map(|x| x.into_owned());
            }
            Some("--nvenc") => {
                input_args = Some("-hwaccel cuda".to_owned());
                output_args = Some("-c:v h264_nvenc".to_owned());
            }
            Some("-o") => {
                let arg = collect_opt_stringed_arg(&mut args, false);
                out_path = arg.map(|x| x.into_owned());
            }
            Some(x) => {
                if TWITCH_VOD_URL_REGEX.is_match(x) {
                    url = x;
                } else {
                    println!("Unknown download option.");
                }
            }
            None => break,
        }
    }

    let client_id = client_id.ok_or("You must provide a Client ID! Please refer to the README.")?;
    let mut twitch = Twitch::new(client_id, auth);
    let id = Twitch::parse_id_from_url(url)?;
    let out_path = match out_path {
        Some(x) => x,
        None => {
            println!("No output path provided... Will use default...");
            format!("{}.mp4", id)
        }
    };

    let mut hls = twitch.get_hls_manifest(url)?;
    match convert::convert_hls_to_file(
        &mut hls,
        out_path,
        input_args.as_deref(),
        output_args.as_deref(),
    ) {
        Ok(_) => println!("Success!"),
        Err(x) => println!("An error occurred:\n{x}"),
    }
    Ok(())
}

fn settings<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<()> {
    let sub_cmd = args.next().ok_or("No subcommand specified!")?;
    match sub_cmd {
        "token" => {
            let op = args.next().ok_or("No operator specified!")?;
            match op {
                "set" => {
                    let value = args.next().ok_or("No value specified!")?;
                    settings::set(|x| x.auth_token = Some(value.to_owned()))?;
                    println!("Successfully set token.");
                }
                "get" => {
                    let settings = settings::get()?;
                    println!("{:?}", settings.auth_token);
                }
                _ => return Err("Unknown operator!".into()),
            }
        }
        "client-id" => {
            let op = args.next().ok_or("No operator specified!")?;
            match op {
                "set" => {
                    let value = args.next().ok_or("No value specified!")?;
                    settings::set(|x| x.client_id = Some(value.to_owned()))?;
                    println!("Successfully set client-id.");
                }
                "get" => {
                    let settings = settings::get()?;
                    println!("{:?}", settings.client_id);
                }
                _ => return Err("Unknown operator!".into()),
            }
        }
        _ => return Err("Unknown subcommand!".into()),
    }
    Ok(())
}

fn version() -> Result<()> {
    println!("{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn parse_cmd(input: &str) -> Result<()> {
    let mut words = input.split_whitespace();
    let cmd_word = words.next().ok_or("No command specified")?;
    match cmd_word {
        "get" => download(words),
        "settings" => settings(words),
        "version" => version(),
        _ => Err(format!("Unknown command. \"{}\"", cmd_word).into()),
    }?;
    Ok(())
}

fn parse_commands() -> Result<()> {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut stdin_buf = String::default();
        let count = stdin.read_line(&mut stdin_buf)?;
        let input = &stdin_buf[..count];
        if let Err(err) = parse_cmd(input) {
            println!("Error:\n{}", err);
        }
    }
}

fn run_interactive() -> Result<()> {
    parse_commands()
}

fn run_once<'a>(args: impl Iterator<Item = String>) -> Result<()> {
    let mut strbuf = String::with_capacity(20);
    for arg in args {
        strbuf.push_str(&arg);
        strbuf.push(' ');
    }
    parse_cmd(&strbuf)
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    if args.len() > 1 {
        args.next(); // Consume the first arg, as it is the program path.
        run_once(args)
    } else {
        run_interactive()
    }
}
