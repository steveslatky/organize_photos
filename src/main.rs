extern crate rexiv2;
extern crate glob;
use glob::glob;

fn main() {

    let folder = "./test_data/";

    let tag_time = "Exif.Image.DateTimeOriginal";
    //let meta = rexiv2::Metadata::new_from_path("./test_data/IMG_0754.jpg").unwrap();
    //let test =  meta.get_xmp_tags().expect("can't get tages");
    //println!("{:?}", meta.get_exif_tags());

    get_images(folder);
}

fn get_images(path:&str) -> Vec< std::path::PathBuf > {
    let mut imgs = Vec::new();
    for entry in glob(&(path.to_owned() + "*.jpg")).expect("Can not find directory.") {
        match entry {
            Ok(p) => imgs.push(p),
            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }
    imgs
}

