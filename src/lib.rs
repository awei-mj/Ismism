use std::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
    },
    env::Args,
    error::Error,
    fs,
    io::{self, BufWriter, Write},
};
use regex::Regex;
use webbrowser;

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
            "-m" => modify(args.next())?,
            _ => return Err("Wrong arguments. Type 'ismism.exe -h' for help.".into()),
        }
    } else {
        return Err("Please input arguments. Type 'ismism.exe -h' for help.".into());
    }
    Ok(())
}

fn deserialize() -> Result<BTreeMap<String, IsmInfo>, Box<dyn Error>> {
    Ok(fs::read_to_string("D:/cmd/IsmBili")?
        .lines()
        .map(|line| {
            let mut fields = line.split_whitespace();
            (
                fields.next().unwrap().to_string(),
                IsmInfo {
                    title: fields.next().unwrap().to_string(),
                    url: fields.next().unwrap().to_string(),
                },
            )
        })
        .collect())
}

fn serialize(map: BTreeMap<String, IsmInfo>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::options()
        .write(true)
        .truncate(true)
        .open("D:/cmd/IsmBili")?;
    let mut buf = BufWriter::with_capacity(0x100000, file);     // 开启1M的buffer
    map.into_iter().for_each(|(ism, IsmInfo { title, url })| {
        if let Err(err) = buf.write(format!("{} {} {}\n", ism, title, url).as_bytes()) {
            panic!("{}", err);
        }
    });
    Ok(())
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
    let map = deserialize()?;
    match arg {
        Some(ism) => match map.get(&ism) {
            Some(info) => {
                println!("Opening {}...", info.title);
                //1-2开头的某几期未能上传b站
                if info.url.starts_with("BV") {
                    webbrowser::open(&format!("https://www.bilibili.com/video/{}", info.url))
                        .unwrap();
                } else {
                    webbrowser::open(&format!(
                        "https://www.qingting.fm/channels/283734/programs/{}",
                        info.url
                    ))
                    .unwrap();
                }
            }
            None => return Err(format!("Didn't find {}.", ism).into()),
        },
        None => return Err("Argument missed. Please input an ismism.".into()),
    }
    Ok(())
}

fn add() -> Result<(), Box<dyn Error>> {
    let mut map = deserialize()?;
    let mut ism = String::new();
    let mut title = String::new();
    let mut url = String::new();
    print!("ismism: ");
    io::stdout().flush().ok().expect("Could not flush stdout.");
    io::stdin().read_line(&mut ism).unwrap();
    print!("title: ");
    io::stdout().flush().ok().expect("Could not flush stdout.");
    io::stdin().read_line(&mut title).unwrap();
    print!("BV: ");
    io::stdout().flush().ok().expect("Could not flush stdout.");
    io::stdin().read_line(&mut url).unwrap();
    ism = ism.trim().to_string();
    title = title.trim().to_string();
    url = url.trim().to_string();
    check_string(&ism)?;
    check_string(&title)?;
    check_string(&url)?;

    match map.entry(ism) {
        Vacant(vacant) => {
            vacant.insert(IsmInfo { title, url });
            ()
        }
        Occupied(_) => return Err("This entry already exists!".into()),
    }
    serialize(map)?;
    Ok(())
}

fn list() -> Result<(), Box<dyn Error>> {
    let map = deserialize()?;
    map.iter().for_each(|(ism, IsmInfo { title, url })| {
        println!("{:16}{:3$}{:12}", ism, title, url, fmt_len(&title));
    });
    println!("{} entries in total.", map.len());
    Ok(())
}

fn find(arg: Option<String>) -> Result<(), Box<dyn Error>> {
    let map = deserialize()?;
    match arg {
        Some(reg) => map
            .iter()
            .filter(|(ism, _)| Regex::new(&format!("^{}$", reg)).unwrap().is_match(ism))
            .for_each(|(ism, IsmInfo { title, url })| {
                println!("{:16}{:3$}{:12}", ism, title, url, fmt_len(&title));
            }),
        None => return Err("Argument missed. Please input a regex.".into()),
    }
    Ok(())
}

fn modify(arg: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut map = deserialize()?;
    match arg {
        Some(ism) => {
            if let Occupied(mut occupied) = map.entry(ism) {
                let mut title = String::new();
                let mut url = String::new();
                print!("title: ");
                io::stdout().flush().ok().expect("Could not flush stdout.");
                io::stdin().read_line(&mut title).unwrap();
                print!("BV: ");
                io::stdout().flush().ok().expect("Could not flush stdout.");
                io::stdin().read_line(&mut url).unwrap();
                title = title.trim().to_string();
                url = url.trim().to_string();
                check_string(&title)?;
                check_string(&url)?;
                occupied.insert(IsmInfo { title, url });
            } else {
                return Err("This entry does not exist!".into());
            }
        }
        None => return Err("Argument missed. Please input an ismism.".into()),
    }

    serialize(map)?;
    Ok(())
}

fn fmt_len(str: &str) -> usize {
    // 一个中文字符占3字节
    // 中文字符数 = (b_len - u_len) / 2
    72 - (str.len() - str.chars().count()) / 2
}

fn check_string(str: &str) -> Result<(), Box<dyn Error>> {
    if str.is_empty() {
        return Err("Empty string!".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_len_test() {
        let str = "一";
        assert_eq!(fmt_len(str), 71);
    }
}
