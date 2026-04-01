use std::{env, fs, process::Command};
use anyhow::{Context, Result, bail};

fn main() -> Result<()>{

    println!("{}", run_cmd()?);
    check_age(15)?;
    run();

    Ok(())
}

fn run_cmd() -> Result<String>{
    let cmdout = Command::new("ls")
        .arg("-l")
        .output().
        context("Failed to execute command")?;
   
    let stdout = String::from_utf8(cmdout.stdout)
        .context("Invalid utf-8")?;

    Ok(stdout)
}

fn check_age(age: i32) -> Result<()>{
    if age < 18 {
        bail!("Age {} is less than 18 - not allowed", age);
    }

    Ok(())
}

fn run() -> Result<()>{
    let file_path = env::var("CONFIG_PATH")
        .context("Environment variable CONFIG_PATH not found")?;

    let data = fs::read_to_string(&file_path)
        .with_context(|| format!("File path not found: {}", file_path))?;

    print!("Data: {}", data);

    Ok(())
}
