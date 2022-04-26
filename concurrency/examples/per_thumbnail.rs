use error_chain::error_chain;

use std::fs::create_dir_all;
use std::path::Path;

use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{imageops::FilterType, ImageError};
use rayon::prelude::*;

error_chain! {
    foreign_links {
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}
const IMAGE_DIR: &str = "/Users/zhouyou/2022/languages/rust/rust-cookbook-code/concurrency/examples";
const THUMB_DIR: &str = "/Users/zhouyou/2022/languages/rust/rust-cookbook-code/concurrency/thumbnails";

fn main() -> Result<()> {
    let p = "images/*.jpeg";
    let o = String::from(IMAGE_DIR) + "/" + p;
    
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with(o.as_str(), options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.is_empty() {
        error_chain::bail!("No .jpg files found in current directory");
    }

    // 缩略图存储目录
    // let thumb_dir = "thumbnails";
    create_dir_all(THUMB_DIR)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), THUMB_DIR);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            println!("    image: {:?}", path);
            make_thumbnail(path, THUMB_DIR, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures
        .iter()
        .for_each(|x| println!("{}", x.display_chain()));

    println!(
        "{} thumbnails saved successfully",
        files.len() - image_failures.len()
    );
    Ok(())
}

fn make_thumbnail<Source, Distination>(
    original: Source,
    thumb_dir: Distination,
    longest_edge: u32,
) -> Result<()>
where
    Source: AsRef<Path>,
    Distination: AsRef<Path>,
{
    let original_path = &(*original.as_ref());
    
    let img = image::open(original.as_ref())?;
    let file_path = Path::new(thumb_dir.as_ref()).join(original_path.file_name().unwrap());
    println!("thumbnail: {:?}", file_path);
    // println!("original: {:?}", original_path);
    

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
