use clap::Parser;
use qrwiz::config::Config;

fn main() {
    let config = Config::parse();

    println!("{:?}", config);

    std::fs::create_dir_all("test_output").unwrap_or_else(|e| {
        eprintln!("Error creating test directory: {}", e);
        std::process::exit(1);
    });

    if !config.urls.is_empty() {
        for (url, index) in config.urls.into_iter().zip(1..) {
            println!("Generating QR code for URL: {}", url.as_str());

            let name = url.domain().unwrap_or_else(|| {
                eprintln!("Error: URL does not have a domain");
                std::process::exit(1);
            });

            //println!("Domain: {}", name);

            let output_file = format!("test_output/{}_{}.png", name, index);

            let result = qrcode_generator::to_png_to_file(
                url.as_str(),
                config.output.ecc.into(),
                config.output.size,
                output_file,
            );

            match result {
                Ok(_) => {
                    println!("QR code generated successfully!");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
