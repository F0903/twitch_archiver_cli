mod settings;

use std::{borrow::Cow, io::Write, path::Path};
use twitch_archiver::{
    convert,
    twitch::{Twitch, TWITCH_VOD_URL_REGEX},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TWITCH_CLIENT_ID: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko"; // Should probably make user enter this into the program.

fn collect_opt_stringed_arg<'a>(args: &mut impl Iterator<Item = &'a str>) -> Option<Cow<'a, str>> {
    let first = match args.next() {
        Some(x) => x,
        None => return None,
    };

    if first.chars().nth(0).unwrap() != '"' {
        return Some(Cow::Borrowed(&first[1..]));
    }

    let mut strbuf = String::with_capacity(first.len() + 20);
    strbuf.push_str(&first[1..]);
    for arg in args {
        strbuf.push(' ');
        if arg.chars().last().unwrap() == '"' {
            strbuf.push_str(&arg[0..arg.len() - 1]);
            break;
        }
        strbuf.push_str(arg);
    }

    Some(Cow::Owned(strbuf))
}

fn download<'a>(args: impl Iterator<Item = &'a str>) -> Result<()> {
    let mut args = args.peekable();

    let mut auth = settings::get()?.auth_token;
    let mut input_args = None;
    let mut output_args = None;
    let mut url = "";

    loop {
        let arg = args.next();
        match arg {
            Some("--auth") => {
                auth = Some(
                    args.next()
                        .ok_or("Auth option specified, but no token was provided.")?
                        .to_owned(),
                );
            }
            Some("--input_args") => {
                let arg = collect_opt_stringed_arg(&mut args);
                input_args = arg.map(|x| x.into_owned());
            }
            Some("--output_args") => {
                let arg = collect_opt_stringed_arg(&mut args);
                output_args = arg.map(|x| x.into_owned());
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

    let mut twitch = Twitch::new(TWITCH_CLIENT_ID, auth);
    let id = Twitch::parse_id_from_url(url)?;
    let out_filename = format!("{}.mp4", id);
    let out_path = match args.next() {
        Some(x) => Path::new(x),
        None => {
            println!("No output path provided... Will use default...");
            Path::new(&out_filename)
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

fn auth<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<()> {
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
        "auth" => auth(words),
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
