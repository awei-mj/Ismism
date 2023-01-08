use std::env::Args;

struct IsmInfo {
    title: String,
    url: String,
}

pub fn process(mut args: Args) -> Result<(), &'static str> {
    args.next();

    if let Some(arg) = args.next() {
        match arg {
            _ => (),
        }
    } else {
        return Err("请输入参数");
    }

    Ok(())
}