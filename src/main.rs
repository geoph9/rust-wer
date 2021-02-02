extern crate clap;
#[macro_use(c)]
extern crate cute;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use clap::{Arg, App};


// Takes into lists of words and not lists of sentences [u16;
fn wer(refer: &Vec<String>, hypoth: &Vec<String>) -> usize {
    // Edit distance
    // const N: u16 = refer.len();
    // const M: u16 = hypoth.len();
    // let mut D = [[0_usize; N]; M];
    let mut d = vec![vec![0_usize; hypoth.len()]; refer.len()];
    for i in 0..refer.len() {
        for j in 0..hypoth.len() {
            if i == 0 {
                d[0][j] = j;
            } else if j == 0 {
                d[i][0] = i;
            }
        }
    }

    for i in 1..refer.len() {
        for j in 1..hypoth.len() {
            if refer[i-1] == hypoth[j-1] {
                d[i][j] = d[i-1][j-1];
            } else {
                // Array of sub, ins, del
                let tmp = [d[i-1][j-1] + 1, d[i][j-1] + 1, d[i-1][j] + 1];
                d[i][j] = *tmp.iter().min().unwrap();
            }
        }
    }
    d[refer.len()-1][hypoth.len()-1]
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file.");
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
    let my_transcriptions = lines_from_file(
        matches.value_of("MY_TRANSCRIPT").unwrap()
    );
    let original_transcript = lines_from_file(
        matches.value_of("TRUE_TRANSCRIPT").unwrap()
    );
    // let ref_test = "What a bright day man".split_whitespace().map(|s| s.to_string()).collect();
    // let hyp_test = "What a bright".split_whitespace().map(|s| s.to_string()).collect();
    // let test = wer(ref_test, hyp_test);

    // println!("GOT A TEST RESULT: {:?}", test);
    assert_eq!(my_transcriptions.len(), original_transcript.len());
    let mytrans: Vec<Vec<String>> = c![sent.split_whitespace()
                                           .map(|s| s.to_string())
                                           .collect::<Vec<String>>(),
                                       for sent in my_transcriptions];
    let truth: Vec<Vec<String>> = c![sent.split_whitespace()
                                         .map(|s| s.to_string())
                                         .collect::<Vec<String>>(),
                                     for sent in original_transcript];
    let wer_vectors = mytrans
        .iter()
        .zip(truth.iter())
        .map(|(x, y)| wer(x, y))
        .collect::<Vec<usize>>();
    println!("WER VECTORS: {:?}", wer_vectors);
    let final_wer = wer_vectors.iter().sum::<usize>() as f32 / wer_vectors.len() as f32;
    println!("THE WER IS: {:?}", final_wer);


}
