use whiskers_launcher_rs::{
    action::{Action, CopyAction},
    api::extensions::{send_response, ExtensionRequest},
    result::{TitleAndDescriptionResult, WLResult},
    utils::{fuzzy_matches, get_search},
};
use whiskers_palette_rs::{
    color::Color,
    palette::{get_panther_palette, get_tiger_palette},
};

use crate::icons::get_icon;

pub fn handle_results(request: ExtensionRequest) {
    let search = get_search(&request.search_text.unwrap());
    let mut results = Vec::<WLResult>::new();

    let mut color_type = "hex".to_string();

    if search.keyword.is_none() && search.search_text.is_empty() {
        send_response(Vec::new());
    }

    if search.keyword.is_some() {
        color_type = match search.keyword.unwrap().as_str() {
            "hsl" => "hsl".to_string(),
            "rgb" => "rgb".to_string(),
            _ => "hex".to_string(),
        }
    }

    for color in get_panther_palette().colors() {
        if fuzzy_matches(&color.name, &search.search_text)
            || fuzzy_matches("panther", &search.search_text)
        {
            results.push(get_color_result(color, "Panther", &color_type));
        }
    }

    for color in get_tiger_palette().colors() {
        if fuzzy_matches(&color.name, &search.search_text)
            || fuzzy_matches("tiger", &search.search_text)
        {
            results.push(get_color_result(color, "Tiger", &color_type));
        }
    }

    send_response(results);
}

fn get_color_result(color: Color, main_palette: &str, color_type: &str) -> WLResult {
    let color_value = match color_type {
        "hsl" => color.hsl.hsl,
        "rgb" => color.rgb.rgb,
        _ => color.hex,
    };

    WLResult::new_title_and_description(
        TitleAndDescriptionResult::new(
            format!("{} {}", main_palette, color.name),
            &color_value,
            Action::new_copy(CopyAction::new(&color_value)),
        )
        .icon(get_icon(format!(
            "{}{}",
            main_palette.to_owned(),
            color.name
        ))),
    )
}
