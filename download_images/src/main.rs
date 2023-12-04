use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpReqest(reqwest::Error);
    }
}

#[tokio::main]
async fn submain() -> Result<()> {
    // 1. Create temporary directory
    let temp_dir = Builder::new().prefix("PPS").tempdir()?;

    // 2. URL to hit
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    // 3. Make response
    let res = reqwest::get(target).await?;
    println!("Response: {:?}", res);

    // 4. Create destination
    let mut dest = {
        // then:: runs regardless of th success of future || and_then:: runs ONLY if future is successful


         /* <res.url()> => Gets the URL from the response.*/
         /* <.path_segments()> => Splits the URL path into segments. */
         /* <.and_then(|segments| segments.last())> => Gets the last segment (filename). */
         /* <.and_then(|name| if name.is_empty() { None } else { Some(name) })> => Checks if the filename is empty. If it is, returns None. Otherwise, returns Some(filename). */
         /* <.unwrap_or("tmp.bin")> => If the filename is None, uses the default filename "tmp.bin". */
 
        let fname = res
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            // .unwrap();
            .unwrap_or("tmp.bin");
        println!("FilenName: {:?}", fname);
        let fname = temp_dir.path().join(fname);
        println!("FileLocation: {:?}", fname);

        File::create(fname)?
    };

    let content = res.text().await?; // creates the full file path by joining the temporary directory path and the filename.
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}

fn main() -> Result<()> {
    submain()?;
    Ok(())
}
