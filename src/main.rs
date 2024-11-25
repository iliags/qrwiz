use clap::Parser;
use qrwiz::{config::Config, OutputData};

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

    // Container to hold the output data
    let mut output_data: Vec<OutputData> = Vec::new();

    // Create the output directory
    std::fs::create_dir_all(directory.clone()).unwrap_or_else(|e| {
        eprintln!("Error creating output directory: {}", e);
        std::process::exit(1);
    });

    // Process the URLs
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

            output_data.push(OutputData {
                data: url.as_str().as_bytes().to_vec(),
                output_file,
            });
        }
    }

    // Generate the QR codes
    for output in output_data {
        let result = match config.output.image_format {
            qrwiz::config::output::ImageFormat::PNG => qrcode_generator::to_png_to_file(
                output.data,
                config.output.ecc.into(),
                config.output.size,
                &output.output_file,
            ),
            qrwiz::config::output::ImageFormat::SVG => qrcode_generator::to_svg_to_file(
                output.data,
                config.output.ecc.into(),
                config.output.size,
                None::<&str>,
                &output.output_file,
            ),
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {} when generating {}", e, &output.output_file);
            }
        }
    }
}
