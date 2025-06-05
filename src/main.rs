// #![allow(unused)]

use colored::Colorize;
use kawayww_sysinfo as sysinfo;
use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    match &args[..] {
        [] => fetch(),
        ["-h"] => help(),
        args => error(args),
    };
}

fn error(args: &[&str]) {
    let args = args.join(" ");
    eprintln!("Failed to parse args: {}", args.red().bold());
}

fn help() {
    let usage = "USAGE".yellow();
    let options = "OPTIONS".yellow();

    let usage_examples = "kawayww_fetch [OPTIONS]".cyan().bold();

    let help_flag = ["-h", "--help"]
        .map(|x| x.cyan().bold().to_string())
        .join(", ");
    let version_flag = ["-V", "--version"]
        .map(|x| x.cyan().bold().to_string())
        .join(", ");

    let help = format!(
        r#"
{usage}:
    {usage_examples}

{options}:
    {help_flag}                        Print help information
    {version_flag}                     Print version information
    "#
    );
    println!("{}", help);
}

fn fetch() {
    macro_rules! pl {
        ($name:expr, $value:expr) => {
            println!(
                "  {} ~ {}",
                $name.to_string().green(),
                $value.to_string().white()
            )
        };
    }

    let cpuinfo = sysinfo::CPUInfo::new().unwrap();
    let uptime = sysinfo::Uptime::new().unwrap();

    pl!("cpu", format!("{}, {}, {}", cpuinfo.brand(), cpuinfo.core_num().0, cpuinfo.frequency()));
    pl!("tm ", uptime.uptime_format(Some(1)));
}
