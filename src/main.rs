use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(ValueEnum, Clone, PartialEq, Debug)]
enum Answer {
    Yes,
    No,
    Maybe,
}

impl Answer {
    fn str_value(&self) -> &'static str {
        match self {
            Answer::Yes => "yes",
            Answer::No => "no",
            Answer::Maybe => "maybe",
        }
    }
}

/// Get funny gifs from https://yesno.wtf
#[derive(Parser, Debug)]
struct Arg {
    /// Answer either `yes`, `no` or `maybe`
    #[clap(value_enum, hide_possible_values = true, ignore_case = true)]
    answer: Option<Answer>,
}

#[derive(Deserialize, Debug)]
struct Resp {
    image: String,
}

fn request(answer: Option<Answer>) -> minreq::Request {
    let req = minreq::get("https://yesno.wtf/api");
    if let Some(value) = answer.as_ref().map(Answer::str_value) {
        req.with_param("force", value)
    } else {
        req
    }
}

fn main() -> Result<(), main_error::MainError> {
    let arg = Arg::parse();
    let resp: Resp = request(arg.answer).with_timeout(10).send()?.json()?;
    println!("{}", resp.image);
    Ok(())
}
