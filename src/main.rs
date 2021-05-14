use clap::{App, Arg};

fn main() {
    let matches = App::new("fut_rating")
        .version("0.1")
        .about("Calculate FIFA Fut Squad Rating")
        .arg(
            Arg::with_name("with")
                .long("with")
                .short("w")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .value_name("RATINGS")
                .help("ratings already have"),
        )
        .arg(
            Arg::with_name("cand")
                .long("candidates")
                .short("c")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .value_name("RATINGS")
                .help("ratings to try"),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .short("t")
                .required(true)
                .takes_value(true)
                .value_name("RATING")
                .help("target rating"),
        )
        .arg(
            Arg::with_name("limit")
                .long("record-limits")
                .short("l")
                .default_value("10")
                .value_name("INT")
                .help("records limit"),
        )
        .get_matches();

    let have = matches
        .values_of("with")
        .unwrap()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let cand = matches
        .values_of("cand")
        .unwrap()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let target = matches.value_of("target").unwrap().parse::<u32>().unwrap();
    let record_limits = matches
        .value_of("limit")
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap();

    let possibles = fut_rating::generate(&have, &cand, target, record_limits);

    fut_rating::show(cand, possibles);
}
