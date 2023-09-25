use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // Open the first file for reading.
    // unwrap retruns file if it's successful otherwise it will go to panic and prints the error
    let file = File::open("myfile.txt").unwrap();
    let reader = BufReader::new(&file);

    // Create or open the second file for writing.
    let mut second_file = File::create("newfile.txt").unwrap();

    // Iterate over the lines in the first file and write them to the second file.
    for line in reader.lines() {
        match line {
            Ok(line) => {
                writeln!(second_file, "{}", line).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading line from first file: {}", err);
                // You can choose to handle the error or exit the loop here.
                // For example, you could break to stop processing or return early.
                // break;
            }
        }

        // Process the line and write it to the second file.
        // if let Ok(line_content) = line {
        //     if let Err(err) = writeln!(second_file, "{}", line_content) {
        //         eprintln!("Error writing to second file: {}", err);
        //         break; // Handle the error or exit the loop as needed.
        //     }
        // } else {
        //     eprintln!("Error reading line from first file");
        //     break; // Handle the error or exit the loop as needed.
        // }
    }
}
