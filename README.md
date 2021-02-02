# Rust-WER

A simple rust program for calculating the *Word Error Rate*. This is part of my learning process for getting to know Rust. 
Also, I wanted to see how much faster Rust can be when compared to interpreter languages such as Python. The file
`python-equivalent/wer.py` has the exact same algorithm written in Python.

Word Error Rate (WER) is a way to evaluate the performance of Speech-To-Text systems. It takes into account how many 
words are needed to be inserted/deleted or substituted between the predicted text (the output of the ASR system) and the 
ground truth (manually transcribed text). In my implementation I am returning the average WER from every separate sentence.

## Dependencies

- `clap = "2.33.3"` for the command line parsing.
- `cute = "0.3.0"` for easier loops.

## Usage

1. Build the project by running `cargo build` inside the directory (or `cargo build --release` in order to avoid 
   recompiling the code when running `cargo run ...`).
2. If you used the `--release` flag then run the program by `cargo run --release $FILE1 $FILE2` where `$FILE1` is the file
of the predicted transcriptions and `$FILE2` is the file containing the true transcriptions. The format of these files is 
   explained below.
3. You can also run the code by doing `./target/release/rust-wer $FILE1 $FILE2`.
4. In order to time the execution time, I used the `time` command. For example, `time ./target/release/rust-wer $FILE1 $FILE2`.
5. The output WER will be simply printed in the console.

## File Format

The input files must be simple text files that contain one sentence per line. The line numbers must be the same between 
the predicted and the true texts. So, if the predicted text was empty for a certain sentence then you need to leave 
an empty line there. See the `data` directory for examples.

Also, the texts must be already preprocessed, i.e. remove punctuations and convert to lower case.

### Timing the test data

To time the rust program do the following:
```bash
time ./target/release/rust-wer ./data/mytranscripts.txt ./data/truth.txt
```

For me, this took `0.003` seconds.

To time the python script do the following:
```bash
time python main.py ./data/mytranscripts.txt ./data/truth.txt
```

While this took `0.149` seconds. So, even with such a small example the execution-time difference is huge.

## TODO:
- Change input file format (json?)
- Maybe handle punctuations/lowercase