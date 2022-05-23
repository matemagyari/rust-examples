#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io::BufReader;
    use std::io::Error;
    use std::fs;

    #[test]
    fn test_basics() {

        let path: &Path = Path::new("/Users/magma/Downloads/hello.txt");

        fn print(f: File) -> Result<(), Error> {
            for line in BufReader::new(f).lines(){
                println!("{}", line?);
            }
            Ok(())
        }

        fn read(mut file: File) -> Result<String, Error> {
            let mut s = String::new();
            file.read_to_string(&mut s);
            Ok(s)
        }

        // Open the path in read-only mode, returns `io::Result<File>`
        File::open(&path)
            .and_then(read)
            .and_then(|c| {
                println!("{}", c);
                Ok(())
            });

        println!("I'm done");

    }
}