#[macro_use]
extern crate lazy_static;

use imageinfo::ImageInfo;

use filesearch::{find_folders};

use crate::exifreader::{ExifReader, create_exif_reader};


mod imageinfo;
mod error;
mod filesearch;
mod manager;
mod exifreader;

fn main() {
    let exif_reader = create_exif_reader();    

    let image_info = exif_reader.load("test_data/images/01.jpg").unwrap();

    println!("{:?}", image_info);

    let folders = filesearch::find_folders("test_data/folders").unwrap();
    println!("Target folders");
    for folder in &folders.target {
        println!("{:?}", folder);
    }


    println!("Source folders");
    for folder in folders.source {
        println!("{:?}", folder);
    }
}

