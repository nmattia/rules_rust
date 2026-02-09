use std::fs;

#[test]
fn repro() {

    let var = std::env::var("THE_FILES").unwrap();
    let files = var.split(" ");

    for file in files {
        println!("Looking up {:?}", file);
        assert!(fs::exists(file).unwrap());
    }
}
