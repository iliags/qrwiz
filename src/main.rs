use clap::Parser;
use qrwiz::config::Config;

fn main() {
    let config = Config::parse();

    //println!("{:?}", config);

    let directory = match config.output.dir {
        Some(dir) => dir,
        None => ".".to_string(),
    };

    let extension = match config.output.image_format {
        qrwiz::config::output::ImageFormat::PNG => "png",
        qrwiz::config::output::ImageFormat::SVG => "svg",
    };

    std::fs::create_dir_all(directory.clone()).unwrap_or_else(|e| {
        eprintln!("Error creating test directory: {}", e);
        std::process::exit(1);
    });

    // TODO: Make a container which houses all data to have QR codes generated

    if !config.urls.is_empty() {
        for (url, index) in config.urls.into_iter().zip(1..) {
            println!("Generating QR code for URL: {}", url.as_str());

            let name = match config.output.name.clone() {
                Some(name) => name,
                None => url
                    .domain()
                    .unwrap_or_else(|| {
                        eprintln!("Error: URL does not have a domain");
                        std::process::exit(1);
                    })
                    .to_string(),
            };

            let output_file = format!("{}/{}_{}.{}", directory, name, index, extension);

            println!("Output file: {}", output_file);

            let result = match config.output.image_format {
                qrwiz::config::output::ImageFormat::PNG => qrcode_generator::to_png_to_file(
                    url.as_str(),
                    config.output.ecc.into(),
                    config.output.size,
                    output_file,
                ),
                qrwiz::config::output::ImageFormat::SVG => qrcode_generator::to_svg_to_file(
                    url.as_str(),
                    config.output.ecc.into(),
                    config.output.size,
                    None::<&str>,
                    output_file,
                ),
            };

            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
