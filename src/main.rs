use std::{env, process::{Command, exit}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        panic!("Atleast 6 Argument is Required!");
    }

    let input_file: &String = &args[2];

    let quality: &String = &args[4];

    let output_file: &String = &args[6];
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            input_file,
            "-vf",
            &format!("scale={}", quality),
            output_file,
        ])
        .output()
        .expect("Caught an error while processing video");
    
    if output.status.success() {
        println!("Video Conversion Succeced");
    } else {
        eprintln!("Video Conversion Error");
        exit(0);
    }
}
