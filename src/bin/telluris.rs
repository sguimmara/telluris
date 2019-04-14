use simplelog::*;
use telluris::app::App;

fn main() {
    let mut logconfig = Config::default();
    logconfig.location = None;
    logconfig.target = None;
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Trace, logconfig).unwrap()]).unwrap();
    let mut app = App::default();
    app.run();
}
