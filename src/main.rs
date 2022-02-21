mod application;
use application::*;

use crate::application::application::Application;
fn main() {
    let mut app = Application::default();
    println!("{:?}", app);
    app.configure(
        Application::set_name("set by opt")
    )
        .configure(
            Application::set_version(version::Version::V2)
        );
    println!("{:?}", app);
}
