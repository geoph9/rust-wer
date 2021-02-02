extern crate clap;
#[macro_use(c)]
extern crate cute;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process::exit,
};
use clap::{Arg, App, SubCommand};

// Takes into lists of words and not lists of sentences [u16;
fn wer(refer: Vec<String>, hypoth: Vec<String>) -> usize {
    // Edit distance
    // const N: u16 = refer.len();
    // const M: u16 = hypoth.len();
    // let mut D = [[0_usize; N]; M];
    let mut D = vec![vec![0_usize; hypoth.len()]; refer.len()];
    println!("D IS: {:?}", D);
    for i in 0..refer.len() {
        for j in 0..hypoth.len() {
            if i == 0 {
                D[0][j] = j;
            } else if j == 0 {
                D[i][0] = i;
            }
        }
    }

    for i in 1..refer.len() {
        for j in 1..hypoth.len() {
            if refer[i-1] == hypoth[j-1] {
                D[i][j] = D[i-1][j-1];
            } else {
                // Array of sub, ins, del
                let tmp = [D[i-1][j-1] + 1, D[i][j-1] + 1, D[i-1][j] + 1];
                D[i][j] = *tmp.iter().min().unwrap();
            }
        }
    }
    D[refer.len()-1][hypoth.len()-1]
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn main() {
    let matches = App::new("WER Calculator")
                                  .version("1.0")
                                  .author("George K.")
                                  .about("Parses to transcript files. You should provide two text files \
                                  that have corresponding lines meaning that the first line from your \
                                  transcripts file should be a transcript for the same audio as the first \
                                  line of the original transcripts.")
                                  .arg(Arg::with_name("MY_TRANSCRIPT")
                                       .help("The transcript which is taken from your ASR service")
                                       .required(true)
                                       .index(1))
                                  .arg(Arg::with_name("TRUE_TRANSCRIPT")
                                       .help("The original transcript.")
                                       .required(true)
                                       .index(2))
                                  .arg(Arg::with_name("verbose")
                                       .short("v")
                                       .multiple(true)
                                       .help("Sets the level of verbosity"))
                                  .get_matches();
    println!("The first argument is {}", matches.value_of("MY_TRANSCRIPT").unwrap());
    // if args.len() != 2 {
    //     println!("You should provide 2 arguments:\n\t1. Your transcripts\n\t2. The original text");
    //     exit(1);
    // }
    let mut my_transcriptions = lines_from_file(
        matches.value_of("MY_TRANSCRIPT").unwrap()
    );
    let mut original_transcript = lines_from_file(
        matches.value_of("TRUE_TRANSCRIPT").unwrap()
    );
    let ref_test = "What a bright day man".split_whitespace().map(|s| s.to_string()).collect();
    let hyp_test = "What a bright".split_whitespace().map(|s| s.to_string()).collect();
    let test = wer(ref_test, hyp_test);

    println!("GOT A TEST RESULT: {:?}", test);

    // let vec: Vec<f32> = c![wer(sent.split(" ")), for sent in my_transcriptions];

}
