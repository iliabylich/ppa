use build_deb_package::{args::paths_from_args, config::Config, green, strategist::Strategist};

fn main() {
    for path in paths_from_args() {
        green!("Explaining {path:?}");
        let config = Config::read(path);
        let plan = Strategist::make_plan(config);
        plan.explain();
    }
}
