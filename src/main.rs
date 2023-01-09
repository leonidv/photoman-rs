use imageinfo::ImageInfo;

use std::path::Path;
use std::ffi::OsStr;

mod imageinfo;
mod error;
mod filesearch;

fn main() {
    // let o_image_info = ImageInfo::read_exif("images/01.jpg");
    // match o_image_info {
    //     Ok(image_info) => println!("{:#?}", image_info),
    //     Err(e) => println!("{}", e)
    // }

    let p = Path::new("/mnt/photo/2020/2020-01-09");
    let o_last_comp = p.components().last();
    match o_last_comp {
        Some(c) => {
            let v = c.as_os_str().to_str().unwrap_or("error");
            println!("{}",v)
        },
        None => println!("error")
    }
}

