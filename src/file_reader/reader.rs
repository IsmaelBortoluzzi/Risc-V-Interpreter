use {
    std::{
        fs::File,
        io::{self, BufRead, BufReader},
        path::Path,
        env,
    },
    crate::run::run::get_start,
};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


pub fn file_reader() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(args[1].as_str());
    let lines: Vec<String> = lines_from_file(filename).expect("Could not load lines");
    
    return lines;

}


pub fn normalize_dotmain(lines: &mut Vec<String>) {
    let mut line: usize = get_start(lines);
    while line < lines.len() {
        lines[line] = lines[line].replace(", ", ",");
        line += 1;
    }
}