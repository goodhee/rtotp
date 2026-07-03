use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Serialize, Deserialize, Clone)]
struct Otp {
    issuer: String,
    account_name: String,
    secret: String,
}

#[derive(Parser)]
#[command(
    name = "rtotp",
    version,
    about = "Rust one-Time Password - a terminal TOTP (RFC 6238) client",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show the current 6-digit code for entry <index> (see `list`)
    Code { index: usize },
    /// Add a new OTP secret
    Add,
    /// List registered secrets
    List,
    /// Remove an entry (prompts if <index> omitted)
    Remove { index: Option<usize> },
    /// Remove all entries
    Clear,
}

fn config_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("could not determine home directory")?
        .join(".rtotplist"))
}

fn load() -> Result<Vec<Otp>> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path)?;
    if data.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&data).context("failed to parse ~/.rtotplist")
}

fn save(list: &[Otp]) -> Result<()> {
    let path = config_path()?;
    fs::write(&path, serde_json::to_string_pretty(list)?)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .context("failed to set ~/.rtotplist permissions")?;
    }
    Ok(())
}

fn totp_for(secret: &str) -> Result<TOTP> {
    let bytes = Secret::Encoded(secret.to_string())
        .to_bytes()
        .map_err(|e| anyhow!("invalid base32 secret: {e:?}"))?;
    TOTP::new(Algorithm::SHA1, 6, 1, 30, bytes).context("failed to build TOTP")
}

fn prompt(label: &str) -> Result<String> {
    print!("{label}");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn print_list(list: &[Otp]) {
    for (i, otp) in list.iter().enumerate() {
        println!("{{{}}} {}:{}", i + 1, otp.issuer, otp.account_name);
    }
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    // gtp-style shortcut: `rtotp 1` behaves like `rtotp code 1`.
    if args.len() == 2 && args[1].parse::<usize>().is_ok() {
        args.insert(1, "code".to_string());
    }

    match Cli::parse_from(args).command {
        Command::Code { index } => {
            let list = load()?;
            let pos = index.checked_sub(1).context("index starts at 1")?;
            let otp = list
                .get(pos)
                .with_context(|| format!("no entry #{index} (see `rtotp list`)"))?;
            println!("{}", totp_for(&otp.secret)?.generate_current()?);
        }
        Command::Add => {
            let issuer = prompt("Step 1/3) Issuer: ")?;
            let account_name = prompt("Step 2/3) Account Name: ")?;
            let secret = rpassword::prompt_password("Step 3/3) Secret (hidden): ")?
                .trim()
                .to_string();
            totp_for(&secret).context("secret is not valid base32")?;
            let mut list = load()?;
            list.push(Otp {
                issuer,
                account_name,
                secret,
            });
            save(&list)?;
            println!("added");
        }
        Command::List => {
            let list = load()?;
            if list.is_empty() {
                println!("nothing registered");
            } else {
                print_list(&list);
            }
        }
        Command::Remove { index } => {
            let mut list = load()?;
            if list.is_empty() {
                println!("nothing registered");
                return Ok(());
            }
            let idx = match index {
                Some(i) => i,
                None => {
                    print_list(&list);
                    prompt("\nRemove which #: ")?
                        .parse()
                        .context("not a number")?
                }
            };
            if idx < 1 || idx > list.len() {
                return Err(anyhow!("#{idx} is out of range"));
            }
            list.remove(idx - 1);
            save(&list)?;
            println!("removed");
        }
        Command::Clear => {
            if prompt("Clear ALL secrets? [y/N]: ")?.eq_ignore_ascii_case("y") {
                save(&[])?;
                println!("cleared");
            }
        }
    }
    Ok(())
}
