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
fn wer(hypothesis: &Vec<String>, reference: &Vec<String>) -> f32 {
    // Edit distance
    // const N: u16 = refer.len();
    // const M: u16 = hypoth.len();
    // let mut D = [[0_usize; N]; M];
    if hypothesis.len() == 0 {
        return 1_f32
    }
    let mut d = vec![vec![0_usize; hypothesis.len()+1]; reference.len()+1];
    for i in 0..(reference.len()+1) {
        for j in 0..(hypothesis.len()+1) {
            if i == 0 {
                d[0][j] = j;
            } else if j == 0 {
                d[i][0] = i;
            }
        }
    }

    for i in 1..(reference.len()+1) {
        for j in 1..(hypothesis.len()+1) {
            if reference[i-1] == hypothesis[j-1] {
                d[i][j] = d[i-1][j-1];
            } else {
                // Array of substitutions, insertions, deletions
                let tmp = [d[i-1][j-1] + 1, d[i][j-1] + 1, d[i-1][j] + 1];
                d[i][j] = *tmp.iter().min().unwrap();
            }
        }
    }
    d[reference.len()][hypothesis.len()] as f32 / reference.len() as f32
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
    // Read the transcripts and convert to a vector of strings
    let my_transcriptions = lines_from_file(
        matches.value_of("MY_TRANSCRIPT").unwrap()
    );
    // Read the true labels and convert to a vector of strings
    let original_transcript = lines_from_file(
        matches.value_of("TRUE_TRANSCRIPT").unwrap()
    );

    // Make sure that both texts have the same number of lines.
    // The texts should match 1-1.
    assert_eq!(my_transcriptions.len(), original_transcript.len());
    // Split the sentences (on space) and save the result in a vector of vectors of the words
    let my_transcripts: Vec<Vec<String>> = c![sent.split_whitespace()
                                           .map(|s| s.to_string())
                                           .collect::<Vec<String>>(),
                                       for sent in my_transcriptions];
    let truth: Vec<Vec<String>> = c![sent.split_whitespace()
                                         .map(|s| s.to_string())
                                         .collect::<Vec<String>>(),
                                     for sent in original_transcript];
    // Get the wer scores for each sentence couple
    let wer_vectors = my_transcripts
        .iter()
        .zip(truth.iter())
        .map(|(x, y)| wer(x, y))
        .collect::<Vec<f32>>();
    // Simply average the WER scores
    let final_wer = wer_vectors.iter().sum::<f32>() / wer_vectors.len() as f32;
    println!("Output WER: {:?}", final_wer);


}
