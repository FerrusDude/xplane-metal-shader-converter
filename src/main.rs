use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write, BufWriter};
use png::{Decoder,Encoder,HasParameters};

fn main() {
    let t: &[_] = &[' ','\n'];
    let mut input = String::new();
    print!("Enter Folder Path: ");
    let _= stdout().flush();
    stdin().read_line(&mut input).expect("\nError - Path not provided");
    let path_string = input.trim_end_matches(t);
    let path_config = format!("{}/config.txt",path_string);

    let config_path = Path::new(&path_config);
    eprintln!("{:?}", config_path);
    let config_file = fs::read_to_string(config_path).expect("\nError - config.txt not found.");
    let config: Vec<String> = config_file.split('\n').map(|x|x.trim_end_matches(t).to_string()).collect::<Vec<String>>();

    if config.len() != 3 {
        panic!("\nError - Three files required. Normal, Metal, Roughness");
    }


    let uv_norm: String = format!("{}/{}",path_string, config[0]).to_string();
    let uv_metal: String = format!("{}/{}",path_string, config[1]).to_string();
    let uv_rough: String = format!("{}/{}",path_string, config[2]).to_string();

    let (uv_norm_info, uv_norm_data) = get_png(uv_norm);
    let n = uv_norm_info.buffer_size();
    let (uv_metal_info, uv_metal_data) = get_png(uv_metal);
    let (uv_rough_info, uv_rough_data) = get_png(uv_rough);

    let mut norm_ctr: usize = 0;
    let mut other_ctr: usize = 0;

    let norm_inc = match uv_norm_info.color_type {
        png::ColorType::RGB => 3,
        png::ColorType::RGBA => 4,
        _ => {
            panic!("\nError - Normal is not an RGB or RGBA PNG")
        },
    };

    if uv_metal_info.color_type != png::ColorType::Grayscale || uv_rough_info.color_type != png::ColorType::Grayscale {
        match uv_metal_info.color_type {
            png::ColorType::Grayscale => {
                panic!("\nError - Roughness is not a Grayscale PNG");
            },
            _ => {
                panic!("\nError - Metallic is not a Grayscale PNG");
            },
        }
    }

    if (uv_norm_info.width == uv_metal_info.width && uv_norm_info.width == uv_rough_info.width) && (uv_norm_info.height == uv_metal_info.height && uv_norm_info.height == uv_rough_info.height) {
        let path_new = format!("{}/uv_metalized.png", path_string);
        let path = Path::new(&path_new);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, uv_norm_info.width, uv_norm_info.height); // 2048px * 2048px
        encoder.set(png::ColorType::RGBA).set(uv_norm_info.bit_depth);
        let mut writer = encoder.write_header().unwrap();

        let s: u32 = 4 * uv_norm_info.width * uv_norm_info.height;
        let mut data = Vec::with_capacity(s as usize);

        while norm_ctr < n {
            data.push(uv_norm_data[norm_ctr]);
            data.push(uv_norm_data[norm_ctr+1]);
            data.push(uv_metal_data[other_ctr]);
            data.push(uv_rough_data[other_ctr]);
            norm_ctr += norm_inc;
            other_ctr += 1;
        }

        writer.write_image_data(&data).expect("\nError - Data Failed to Write");

        println!("\nImage successfully combined!!");
    } else {
        eprintln!("\nError - Source Images are not the same dimensions.");
    }
    
}

fn get_png(path: String) -> (png::OutputInfo,Vec<u8>) {
    eprintln!("\n{}",path);
    let decoder = Decoder::new(File::open(path).expect("\nError - Decoder failed to initialize"));
    let (info, mut reader) = decoder.read_info().expect("\nError - Decoder failed to read");
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("\nError - Reader failed to advance to next frame");
    (info,Vec::from(buf))
}