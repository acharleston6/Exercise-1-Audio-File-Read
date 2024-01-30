use std::fs::File;
use std::io::Write;

use std::i16;
use hound;


fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = std::env::args().collect();
 
    // TODO: your code here
    let wavFile = &args[1];
    let output = &args[2];
    println!("waveFile: {}", wavFile);
    println!("output: {}", output);      

    // Open the input wave file and determine number of channels
    // TODO: your code here; see `hound::WavReader::open`.
    let mut sound = hound::WavReader::open(wavFile).unwrap();
    let mut soundSpecs = sound.spec();
    let channels = soundSpecs.channels;
    println!("channels: {}", channels);    
    let mut addSound = &sound;

    // Read audio data and write it to the output text file (one column per channel)
    // TODO: your code here; we suggest using `hound::WavReader::samples`, `File::create`, and `write!`.
    //       Remember to convert the samples to floating point values and respect the number of channels!

    let mut current = 0;

    let mut file = File::create(output).unwrap();
    let mut fileAdd = &file;

    for sample in sound.samples::<i16> () {
        if current % channels == 0 {
            //write()
            file.write(sample.unwrap().to_string().as_bytes());
        } else {
            file.write(sample.unwrap().to_string().as_bytes());
        }
        current += 1;
    }
    

    



}
