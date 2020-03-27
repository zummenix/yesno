use serde::Deserialize;
use std::error;

#[derive(Deserialize, Debug)]
struct Resp {
    answer: String,
    forced: bool,
    image: String
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let resp: Resp = minreq::get("https://yesno.wtf/api").send()?.json()?;
    println!("{}", resp.image);
    Ok(())
}
