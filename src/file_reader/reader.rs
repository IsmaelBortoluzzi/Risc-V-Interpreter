use {
    std::{
        fs::File,
        io::{self, BufRead, BufReader},
        path::Path,
        env,
    },
};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


pub fn reader() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(args[1].as_str());
    
    let lines: Vec<String> = lines_from_file(filename).expect("Could not load lines");
    
    return lines;

}