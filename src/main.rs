use serde::Deserialize;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Answer {
        Yes,
        No,
        Maybe
    }
}

#[derive(StructOpt, Debug)]
/// Get funny gifs from https://yesno.wtf
struct Opt {
    #[structopt(possible_values = &Answer::variants())]
    #[structopt(hide_possible_values = true, case_insensitive = true)]
    /// Answer either `yes`, `no` or `maybe`
    answer: Option<Answer>,
}

#[derive(Deserialize, Debug)]
struct Resp {
    image: String,
}

fn url(answer: Option<Answer>) -> String {
    let mut base_url = String::from("https://yesno.wtf/api");
    match answer {
        Some(Answer::Yes) => base_url.push_str("?force=yes"),
        Some(Answer::No) => base_url.push_str("?force=no"),
        Some(Answer::Maybe) => base_url.push_str("?force=maybe"),
        None => (),
    }
    base_url
}


fn main() -> Result<(), main_error::MainError> {
    let opt = Opt::from_args();
    let resp: Resp = minreq::get(url(opt.answer))
        .with_timeout(10)
        .send()?
        .json()?;
    println!("{}", resp.image);
    Ok(())
}
