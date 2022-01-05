use clap::{ArgEnum, Parser};
use serde::Deserialize;

#[derive(ArgEnum, Clone, PartialEq, Debug)]
enum Answer {
    Yes,
    No,
    Maybe,
}

/// Get funny gifs from https://yesno.wtf
#[derive(Parser, Debug)]
struct Opt {
    /// Answer either `yes`, `no` or `maybe`
    #[clap(arg_enum, hide_possible_values = true, ignore_case = true)]
    answer: Option<Answer>,
}

#[derive(Deserialize, Debug)]
struct Resp {
    image: String,
}

fn url(answer: Option<Answer>) -> String {
    let mut base_url = String::from("https://yesno.wtf/api");
    match answer {
        Some(Answer::Yes) => base_url += "?force=yes",
        Some(Answer::No) => base_url += "?force=no",
        Some(Answer::Maybe) => base_url += "?force=maybe",
        None => (),
    }
    base_url
}

fn main() -> Result<(), main_error::MainError> {
    let opt = Opt::parse();
    let resp: Resp = minreq::get(url(opt.answer))
        .with_timeout(10)
        .send()?
        .json()?;
    println!("{}", resp.image);
    Ok(())
}
