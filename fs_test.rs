use fs_test::create_dir_test;
use fs_test::create_file_test;

fn main() {
    create_dir_test();
    create_file_test();
}

mod fs_test {
    use std::fs;
    use std::io::Error;
    use std::path::Path;
    pub fn create_dir_test() {
        let path: &str = "./test";
        let dir_path: &Path = Path::new(path);
        if dir_path.exists() {
            println!("The test directory already exists! Skipping creation ...");
            return;
        }
        let create_dir_test_res: Result<(), Error> = fs::create_dir("./test");
        if create_dir_test_res.is_ok() {
            println!("created new directory")
        } else {
            println!(
                "There was a problem creating the directory. {:?}",
                create_dir_test_res.err()
            );
        }
    }
    pub fn create_file_test() {
        let path: &str = "./test/test.txt";
        let text: &str = "this is a test";
        _ = fs::write(path, text);
    }
}
