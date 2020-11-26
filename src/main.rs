use std::{
    io::{self, Write, prelude::*, SeekFrom},
    thread,
    time::Duration,
    fs::File,
    collections::HashMap,
};

struct IoStats {
    pub mb_read: f64,
    pub mb_write: f64,
}

fn main() {
    let mut prev: HashMap<String, IoStats> = HashMap::new();

    let mut fd = File::open("/proc/diskstats").unwrap();

    loop {
        let mut curr: HashMap<String, IoStats> = HashMap::new();
        // Output string
        let mut output = String::new();
        // Add the header string to the output
        output.push_str("\nDevice\tmb_reads/s\tmb_wrtn/s\n\n");
        // Create a new empty string
        let mut io_data = String::new();
        // Read the content of the file (diskstats) to the io_data string
        fd.read_to_string(&mut io_data).unwrap();
        // Iterate over each line (line is a disk)
        for line in io_data.lines() {
            // Split field (separated by whitespace) and collect them without specific type
            let fields = line.split_whitespace().collect::<Vec<_>>();
            let ds = IoStats {
                mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
                mb_write: fields[9].parse::<f64>().unwrap() / 2048.0,
            };
            // If prev already contains the info we compute the diff to get mb/s
            // Else we add to the print line the "fake" data
            if prev.contains_key(fields[2]) {
                // Get the object from the hasmap
                let pds = prev.get(fields[2]).unwrap();
                // Construct speed line and append it to curr hasmap
                let mb_reads_s = ds.mb_read - pds.mb_read;
                let mb_wrtn_s = ds.mb_write - pds.mb_write;
                // Add the line formatted with color and spacing;
                output.push_str(&format!("\x1b[0;32m{:16}\x1b[0m\x1b[0;34m{:10.2}{:15.2}\x1b[0m\n", fields[2], mb_reads_s, mb_wrtn_s));
                // Insert the current disk data to the curr HashMap
                // The curr will later be saved as prev
                curr.insert(fields[2].to_owned(), ds);
            } else {
                // Add the line with fake data
                output.push_str(&format!("\x1b[0;32m{:16}\x1b[0m\x1b[0;34m{:10.2}{:15.2}\x1b[0m\n", fields[2], 0.00, 0.00));
            }
        }
        // Move the cursor to the start of the file
        fd.seek(SeekFrom::Start(0)).unwrap();
        // Print the result
        writeln!(io::stdout().lock(), "{}", output);

        prev = curr;
        thread::sleep(Duration::from_secs(1));
    }
}
