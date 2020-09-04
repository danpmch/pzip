
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

    let mut content_bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut content_bytes)
        .expect("Could not read file contents");

    println!("Total bytes in file: {}", content_bytes.len());

    let mut v = Vec::<u8>::new();
    let size: u32 = content_bytes
        .len()
        .try_into()
        .expect("Error, file is too big for the Rust image library to handle");
    v.extend(&size.to_be_bytes());
    v.extend(content_bytes);

    let (width, height): (u32, u32) = {
        let total_pixels = ((v.len() as f64) / 3.0).ceil();
        let w = total_pixels.sqrt().floor();
        let h = (total_pixels / w).ceil();
        (w as u32, h as u32)
    };

    println!("Total bytes to write {}", v.len());
    println!("Computed Width, Height: ( {}, {} )", width, height);
    println!("Total image bytes: {}", width * height * 3);

    let missing_bytes = (width * height * 3) - (v.len() as u32);

    v.extend(vec![0; missing_bytes.try_into().unwrap()].iter());
    //println!("{:x?}", v);

    let output_file = File::create(filename.to_owned() + ".png")
        .expect("Could not open output file test.png");
    let png = PngEncoder::new(output_file);
    png.encode(
      &v,
      width,
      height,
      image::ColorType::Rgb8
    ).expect("Error writing image file");
}

fn decode(filename: &String) {
    let f = File::open(filename)
        .expect("Could not open test.txt");
    let png = PngDecoder::new(f).expect("Could not decode file as PNG");
    let mut contents = vec![0u8; png.total_bytes().try_into().unwrap()];

    //println!("Total image bytes: {}", png.total_bytes());
    //println!("Size of buffer: {}", contents.len());

    png.read_image(&mut contents)
        .expect("Error reading image");
    //println!("{:x?}", contents);

    let size_bytes: [u8; 4] = contents[0..4]
        .try_into()
        .expect("Error, could not read data length");
    let size: usize = u32::from_be_bytes(size_bytes).try_into().unwrap();
    //println!("Size header: {}", size);
    let data = &contents[4..(4 + size)];
    //println!("{:x?}", data);

    let mut output_file = File::create(filename.to_owned() + ".decoded")
        .expect("Error opening output file");
    output_file.write(&data).expect("Error writing output file");
}


