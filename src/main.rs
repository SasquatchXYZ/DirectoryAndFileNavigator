fn main() {
    let mut current_dir = std::env::current_dir().unwrap();
    println!("Current director: {current_dir:?}");

    let mut read_dir = std::fs::read_dir(&current_dir).unwrap();
    println!("{read_dir:?}");
    let first = read_dir.nth(1).unwrap().unwrap();
    println!("Path: {:?} Name: {:?}", first.path(), first.file_name());

    current_dir.pop();
    println!("Now moved back to: {current_dir:?}");

    let mut read_dir = std::fs::read_dir(&current_dir).unwrap();
    println!("{read_dir:?}");
    let first = read_dir.nth(1).unwrap().unwrap();
    println!("Path: {:?} Name: {:?}", first.path(), first.file_name());
}
