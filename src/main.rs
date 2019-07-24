use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    if let Ok(lines) = read_lines("Cust.csv") {
        let fileimpx = OpenOptions::new().append(true).create(true).open("foo.txt").expect("fail");
        let mut filex = &fileimpx;
        for (idx, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if idx == 0 {
                    filex.write_all("UPDATE CUSTOMER;uid[unique=true];customerId\n".as_bytes()).expect("fail");
                }
                let spt = ip.as_str().split(',');
                let v: Vec<&str> = spt.collect();
                filex.write_all(format!(";{};{}\n",v[3].replace("\"",""),v[4].replace("\"","")).as_bytes()).expect("fail");
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
