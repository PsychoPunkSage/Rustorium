// To work with Files/Filesystem. \\

/**
 * Accessing files: Opening, closing, reading from, writing to, and deleting files.
 * Creating directories: Creating, listing, and deleting directories.
 * Metadata manipulation: Getting and setting file permissions, timestamps, and other attributes.
 * File system walks: Recursively listing files and directories within a path.
 * Temporary files: Creating and managing temporary files safely.
 * Buffering and encoding: Efficiently reading and writing data with buffers and character encodings.
 */
use std::fs;
/**
 * Files: Open and read data from files on disk, including text and binary formats.
 * Standard input: Read data from the user's keyboard, often used for command-line applications.
 * Network connections: Establish connections and read data from sockets or other network streams.
 * In-memory buffers: Read data directly from memory buffers without needing external sources.
 */
use std::io;
use std::process;
use std::path::Path;

fn main() {
    // Exit cleanly..
    process::exit(logic());  // asa logic() returns an integer (say 1). Program will exit with 'exit code 1'
}

fn logic() -> i32 {
    // 1. Collect User Input from CLI
    let args: Vec<_> = std::env::args().collect();

    println!("________");
    println!("| ARGS | :: ${:?}", args);
    println!("--------");

    // 2. If args < 2, then treat the issue + added `Usage``
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // 3. create 'new Path object' using 2nd argument
    let f_name = Path::new(&*args[1]);
    // 4. Open the file (use `fs`)
    let file = fs::File::open(&f_name).unwrap();

    // 5. Using the Archive reader fn:: open zip file for (reading + writing)
    let mut archive = zip::ZipArchive::new(file).unwrap();

    println!("___________");
    println!("| ARCHIVE |\n----------- \n:: ${:?} \nlength :: {:?}", archive, archive.len());
    println!("");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // println!("File {} ==> Name: {:?}, Size: {:?}", i, file.name().unwrap(), file.size());s

        // Setting Path where file will be extracted...
        let out_path = match file.enclosed_name() { // Resistant: Path_based exploit {Ensure the file path is safe to use as a Path.}
            Some(path) => path.to_owned(), // .to_owned():: Cloning from zip file
            None => continue,
        };

        {                                                             
            let comment = file.comment();
            if !comment.is_empty() {
                println!("FIle {} comment: {}", i, comment);
            }
        }

        // The zip may be Folder...
        if (*file.name()).ends_with('/') { // * -> ref is taken becoz I don't want to exhaust "file" object.
            println!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                file.size()
            );

            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // if there is no parent for those files, create a new directory
            let mut out_file = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }

        // Get and Set permissions for the extracted files: only for unix based systems.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}
