use results::handle_results;
use whiskers_launcher_rs::api::extensions::get_extension_request;

mod results;
mod icons;

fn main() {
    let request = get_extension_request();

    handle_results(request);
}
