//REQUIRED FOR WINDOWS EXTENSIONS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use icons::get_icon;
use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_core::{
    features::{core::extensions::get_extension_request, extensions::send_search_results},
    results::{CopyTextAction, ResultAction, SearchResult, SearchResults},
    utils::get_search_query,
};
use whiskers_palette_rs::{
    color::Color,
    palette::{get_panther_palette, get_tiger_palette},
};

mod icons;

fn main() {
    let request = get_extension_request();
    let search_query = get_search_query(&request.search_text.unwrap());
    let mut results = Vec::<SearchResult>::new();
    let sniffer = Sniffer::new();

    let mut color_type = "hex".to_string();

    if search_query.keyword.is_none() && search_query.search_text.is_empty() {
        send_search_results(SearchResults::new_list_results(vec![]));
    }

    if search_query.keyword.is_some() {
        color_type = match search_query.keyword.unwrap().as_str() {
            "hsl" => "hsl".to_string(),
            "rgb" => "rgb".to_string(),
            _ => "hex".to_string(),
        }
    }

    for color in get_panther_palette().colors() {
        let full_color_name = format!("Panther {}", &color.name);

        if sniffer.matches(&full_color_name, &search_query.search_text) {
            results.push(get_color_result(color, "Panther", &color_type));
        }
    }

    for color in get_tiger_palette().colors() {
        let full_color_name = format!("Tiger {}", &color.name);

        if sniffer.matches(&full_color_name, &search_query.search_text) {
            results.push(get_color_result(color, "Tiger", &color_type));
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}

fn get_color_result(color: Color, main_palette: &str, color_type: &str) -> SearchResult {
    let color_value = match color_type {
        "hsl" => color.hsl.hsl,
        "rgb" => color.rgb.rgb,
        _ => color.hex,
    };

    SearchResult::new(
        format!("{} {}", main_palette, color.name),
        ResultAction::new_copy_text_action(CopyTextAction::new(&color_value)),
    )
    .set_description(&color_value)
    .set_icon(get_icon(format!(
        "{}{}",
        main_palette.to_owned(),
        color.name
    )))
}
