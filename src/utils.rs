
/// gives a buffered reader to iterate over for the lines of a file
/// {day}.txt in the data folder.
fn get_buffered_reader(day: u8) -> BufReader<File>{
    let path = format!("data/{}.txt", day);
    let input = File::open(path).expect("could not open file!");
    BufReader::new(input)
}
