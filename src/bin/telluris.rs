use simplelog::*;
use telluris::app::App;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap()
    ])
    .unwrap();
    let mut app = App::default();
    app.run();
}
