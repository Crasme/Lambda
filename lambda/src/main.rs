use std::io::Read;

mod libs;

fn main() {
    let path = std::path::Path::new("./../input/main.lb");
    // open the file
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // read the file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    // get the part of definition
    let mut lines = contents.lines();
    let mut definition = String::new();
    let mut in_definition = false;
    while let Some(line) = lines.next() {
        if line.starts_with("// DEFS") {
            in_definition = true;
        }
        if in_definition {
            definition.push_str(line);
            definition.push('\n');
        }
        if line.starts_with("// ENDDEFS") {
            in_definition = false;
        }
    }
    let mut raw_defs: Vec<&str> = definition.split('\n').collect::<Vec<_>>().into_iter().filter(|x| !x.is_empty()).collect();
    let def_vec: Vec<&str> = raw_defs.drain(1..raw_defs.len()-1).collect();
    println!("{:?}", def_vec);
}
