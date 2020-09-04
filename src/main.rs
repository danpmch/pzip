
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env;
use std::convert::TryInto;
use image::png::PngEncoder;
use image::png::PngDecoder;
use image::ImageDecoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage:  pzip [encode|decode] FILE");
        return ();
    }

    let filename = &args[2];

    match args[1].as_str() {
        "encode" => encode(filename),
        "decode" => decode(filename),
        unknown => println!("Unknown command {}", unknown),
    };
}

fn encode(filename: &String) {

    let mut f = File::open(filename)
        .expect("Could not open test.txt");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read file contents");

    println!("{}", contents);

    let mut bytes = contents.into_bytes();

    let (width, height) = {
        let total_pixels = ((bytes.len() as f64) / 3.0).ceil();
        let w: u32 = total_pixels.sqrt() as u32;
        let h: u32 = (total_pixels as u32) / w +
            if total_pixels as u32 % w > 0 { 1 } else { 0 };
        (w, h)
    };

    println!("Total bytes to write {}", bytes.len());
    println!("Computed Width, Height: ( {}, {} )", width, height);
    println!("Total image bytes: {}", width * height * 3);

    let missing_bytes = (width * height * 3) - (bytes.len() as u32);
    bytes.extend(vec![0; missing_bytes.try_into().unwrap()].iter());

    let output_file = File::create(filename.to_owned() + ".png")
        .expect("Could not open output file test.png");
    let png = PngEncoder::new(output_file);
    png.encode(
      &bytes,
      width,
      height,
      image::ColorType::Rgb8
    ).expect("Error writing image file");
}

fn decode(filename: &String) {
    let f = File::open(filename)
        .expect("Could not open test.txt");
    let png = PngDecoder::new(f).expect("Could not decode file as PNG");
    let mut contents = vec![0 as u8; png.total_bytes().try_into().unwrap()];

    println!("Total image bytes: {}", png.total_bytes());
    println!("Size of buffer: {}", contents.len());

    png.read_image(&mut contents)
        .expect("Error reading image");

    let mut output_file = File::create(filename.to_owned() + ".decoded")
        .expect("Error opening output file");
    output_file.write(&contents).expect("Error writing output file");
}


