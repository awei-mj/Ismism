use std::{fs, env::Args, collections::BTreeMap, error::Error};
use webbrowser;
use regex::Regex;

struct IsmInfo {
    title: String,
    url: String,
}

pub fn process(mut args: Args) -> Result<(), Box<dyn Error>> {
    args.next();

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" => help(),
            "-o" => open(args.next())?,
            "-a" => add()?,
            "-l" => list()?,
            "-f" => find(args.next())?,
            "-m" => modify()?,
            _ => return Err("Wrong arguments. Type 'ismism.exe -h' for help.".into()),
        }
    } else {
        return Err("Please input arguments. Type 'ismism.exe -h' for help.".into());
    }

    Ok(())
}

fn load() -> Result<BTreeMap<String, IsmInfo>, Box<dyn Error>> {
    Ok(fs::read_to_string("D:/cmd/IsmBili")?.lines().map(|line| {
        let mut fields = line.split_whitespace();
        (fields.next().unwrap().to_string(),
        IsmInfo {
            title: fields.next().unwrap().to_string(),
            url: fields.next().unwrap().to_string()
        })
    }).collect())
}

fn help() {
    println!("Usage: ismism.exe [options]");
    println!("Options:");
    println!("  -o [ismism]\topen");
    println!("  -f [regex]\tfind by regex");
    println!("  -m [ismism]\tmodify");
    println!("  -a \t\tadd");
    println!("  -l \t\tlist");
    println!("  -h \t\thelp");
}

fn open(arg: Option<String>) -> Result<(), Box<dyn Error>> {
    let map = load()?;
    match arg {
        Some(ism) => match map.get(&ism) {
            Some(info) => {
                println!("Opening {}...", info.title);
                //1-2开头的某几期未能上传b站
                if info.url.starts_with("BV"){
                    webbrowser::open(&format!("https://www.bilibili.com/video/{}", info.url))
                        .unwrap();
                } else {
                    webbrowser::open(&format!("https://www.qingting.fm/channels/283734/programs/{}", info.url)).unwrap();
                }
            },
            None => return Err(format!("Didn't find {}.", ism).into()),
        },
        None => return Err("Argument missed. Please input an ismism.".into()),
    }
    Ok(())
}

//检查是否已存在
fn add() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn list() -> Result<(), Box<dyn Error>> {
    let map = load()?;
    map.iter().for_each(|(ism, IsmInfo{title, url})| {
        println!("{:16}{:3$}{:12}", ism, title, url, fmt_len(&title));
    });
    println!("{} entries in total.", map.len());
    Ok(())
}

fn find(arg: Option<String>) -> Result<(), Box<dyn Error>> {
    let map = load()?;
    match arg {
        Some(reg) => {
            map.iter().filter(|(ism, _)| Regex::new(&format!("^{}$", reg)).unwrap()
            .is_match(ism)).for_each(|(ism, IsmInfo{title, url})| {
                println!("{:16}{:3$}{:12}", ism, title, url, fmt_len(&title));
            })
        },
        None => return Err("Argument missed. Please input a regex.".into()),
    }
    Ok(())
}

//检查是否不存在
fn modify() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn fmt_len(str: &str) -> usize {
    // 一个中文字符占3字节
    // 中文字符数 = (b_len - u_len) / 2
    72 - (str.len() - str.chars().count()) / 2
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn fmt_len_test() {
        let str = "一";
        assert_eq!(fmt_len(str), 71);
    }
}
