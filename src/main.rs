#[macro_use]
extern crate lazy_static;

use imageinfo::ImageInfo;

use filesearch::{find_folders};

use crate::exifreader::{ExifReader, create_exif_reader};
use crate::manager::Manager;

mod imageinfo;
mod error;
mod filesearch;
mod manager;
mod exifreader;

fn main() {
    let mut exif_reader = create_exif_reader();    

    let image_info = exif_reader.load("test_data/images/01.jpg").unwrap();

    println!("{:?}", image_info);

    let folders = filesearch::find_folders(&"test_data/").unwrap();
    println!("Target folders");
    for folder in &folders.target {
        println!("{:?}", folder);
    }


    println!("Source folders");
    for folder in folders.source {
        println!("{:?}", folder);
    }

    println!("{}","═".repeat(40));

    let _manager = Manager::new("test_data/", true);
    match _manager {
        Ok(mut manager) => manager.arrange_files(&exif_reader),
        Err(e) => eprintln!("{}",e),
    }
    

   // manager::arrange_files(&folders.target, &folders.source, &exif_reader);
}

