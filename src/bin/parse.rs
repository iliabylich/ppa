use build_deb_package::{args::paths_from_args, config::Config, green};

fn main() {
    for path in paths_from_args() {
        green!("Parsing {path:?}");
        let config = Config::read(path);
        println!("{config:#?}");
    }
}
