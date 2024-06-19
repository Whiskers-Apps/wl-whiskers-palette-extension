use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use whiskers_launcher_rs::{
    action::{Action, CopyAction},
    api::extensions::{send_response, ExtensionRequest},
    result::{TextResult, WLResult},
    utils::get_search,
};
use whiskers_palette_rs::palette::{get_panther_palette, get_tiger_palette};

use crate::icons::get_icon;

pub fn handle_results(request: ExtensionRequest) {
    let search = get_search(&request.search_text.unwrap());
    let mut results = Vec::<WLResult>::new();
    let matcher = SkimMatcherV2::default();
    let panther_palette = get_panther_palette();
    let tiger_palette = get_tiger_palette();

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

    if matcher.fuzzy_match("Banana", &search.search_text).is_some() {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.banana.hex,
            "rgb" => &panther_palette.banana.rgb.rgb,
            "hsl" => &panther_palette.banana.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.banana.hex,
            "rgb" => &tiger_palette.banana.rgb.rgb,
            "hsl" => &tiger_palette.banana.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Banana",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-banana")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Banana",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-banana")),
        ));
    }

    if matcher
        .fuzzy_match("Blueberry", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.blueberry.hex,
            "rgb" => &panther_palette.blueberry.rgb.rgb,
            "hsl" => &panther_palette.blueberry.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.blueberry.hex,
            "rgb" => &tiger_palette.blueberry.rgb.rgb,
            "hsl" => &tiger_palette.blueberry.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Blueberry",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-blueberry")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Blueberry",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-blueberry")),
        ));
    }

    if matcher.fuzzy_match("Cherry", &search.search_text).is_some() {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.cherry.hex,
            "rgb" => &panther_palette.cherry.rgb.rgb,
            "hsl" => &panther_palette.cherry.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.cherry.hex,
            "rgb" => &tiger_palette.cherry.rgb.rgb,
            "hsl" => &tiger_palette.cherry.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Cherry",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-cherry")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Cherry",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-cherry")),
        ));
    }

    if matcher.fuzzy_match("Grape", &search.search_text).is_some() {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.grape.hex,
            "rgb" => &panther_palette.grape.rgb.rgb,
            "hsl" => &panther_palette.grape.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.grape.hex,
            "rgb" => &tiger_palette.grape.rgb.rgb,
            "hsl" => &tiger_palette.grape.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Grape",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-grape")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Grape",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-grape")),
        ));
    }

    if matcher.fuzzy_match("Kiwi", &search.search_text).is_some() {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.kiwi.hex,
            "rgb" => &panther_palette.kiwi.rgb.rgb,
            "hsl" => &panther_palette.kiwi.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.kiwi.hex,
            "rgb" => &tiger_palette.kiwi.rgb.rgb,
            "hsl" => &tiger_palette.kiwi.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Kiwi",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-kiwi")),
        ));

        results.push(WLResult::new_text(
            TextResult::new("Tiger Kiwi", Action::new_copy(CopyAction::new(tiger_color)))
                .icon(get_icon("tiger-kiwi")),
        ));
    }

    if matcher
        .fuzzy_match("Tangerine", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.tangerine.hex,
            "rgb" => &panther_palette.tangerine.rgb.rgb,
            "hsl" => &panther_palette.tangerine.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.tangerine.hex,
            "rgb" => &tiger_palette.tangerine.rgb.rgb,
            "hsl" => &tiger_palette.tangerine.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Tangerine",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-tangerine")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Tangerine",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-tangerine")),
        ));
    }

    if matcher
        .fuzzy_match("Neutral", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral.hex,
            "rgb" => &panther_palette.neutral.rgb.rgb,
            "hsl" => &panther_palette.neutral.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral.hex,
            "rgb" => &tiger_palette.neutral.rgb.rgb,
            "hsl" => &tiger_palette.neutral.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralTwo", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_two.hex,
            "rgb" => &panther_palette.neutral_two.rgb.rgb,
            "hsl" => &panther_palette.neutral_two.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_two.hex,
            "rgb" => &tiger_palette.neutral_two.rgb.rgb,
            "hsl" => &tiger_palette.neutral_two.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Two",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-two")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Two",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-two")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralThree", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_three.hex,
            "rgb" => &panther_palette.neutral_three.rgb.rgb,
            "hsl" => &panther_palette.neutral_three.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_three.hex,
            "rgb" => &tiger_palette.neutral_three.rgb.rgb,
            "hsl" => &tiger_palette.neutral_three.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Three",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-three")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Three",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-three")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralFour", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_four.hex,
            "rgb" => &panther_palette.neutral_four.rgb.rgb,
            "hsl" => &panther_palette.neutral_four.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_four.hex,
            "rgb" => &tiger_palette.neutral_four.rgb.rgb,
            "hsl" => &tiger_palette.neutral_four.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Four",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-four")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Four",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-four")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralFive", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_five.hex,
            "rgb" => &panther_palette.neutral_five.rgb.rgb,
            "hsl" => &panther_palette.neutral_five.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_five.hex,
            "rgb" => &tiger_palette.neutral_five.rgb.rgb,
            "hsl" => &tiger_palette.neutral_five.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Five",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-five")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Five",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-five")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralSix", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_six.hex,
            "rgb" => &panther_palette.neutral_six.rgb.rgb,
            "hsl" => &panther_palette.neutral_six.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_six.hex,
            "rgb" => &tiger_palette.neutral_six.rgb.rgb,
            "hsl" => &tiger_palette.neutral_six.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Six",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-six")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Six",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-six")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralSeven", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_seven.hex,
            "rgb" => &panther_palette.neutral_seven.rgb.rgb,
            "hsl" => &panther_palette.neutral_seven.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_seven.hex,
            "rgb" => &tiger_palette.neutral_seven.rgb.rgb,
            "hsl" => &tiger_palette.neutral_seven.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Seven",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-seven")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Seven",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-seven")),
        ));
    }

    if matcher
        .fuzzy_match("NeutralEight", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.neutral_eight.hex,
            "rgb" => &panther_palette.neutral_eight.rgb.rgb,
            "hsl" => &panther_palette.neutral_eight.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.neutral_eight.hex,
            "rgb" => &tiger_palette.neutral_eight.rgb.rgb,
            "hsl" => &tiger_palette.neutral_eight.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Neutral Eight",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-neutral-eight")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Neutral Eight",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-neutral-eight")),
        ));
    }

    if matcher.fuzzy_match("Text", &search.search_text).is_some() {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.text.hex,
            "rgb" => &panther_palette.text.rgb.rgb,
            "hsl" => &panther_palette.text.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.text.hex,
            "rgb" => &tiger_palette.text.rgb.rgb,
            "hsl" => &tiger_palette.text.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Text",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-text")),
        ));

        results.push(WLResult::new_text(
            TextResult::new("Tiger Text", Action::new_copy(CopyAction::new(tiger_color)))
                .icon(get_icon("tiger-text")),
        ));
    }

    if matcher
        .fuzzy_match("TextTwo", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.text_two.hex,
            "rgb" => &panther_palette.text_two.rgb.rgb,
            "hsl" => &panther_palette.text_two.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.text_two.hex,
            "rgb" => &tiger_palette.text_two.rgb.rgb,
            "hsl" => &tiger_palette.text_two.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Text Two",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-text-two")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Text Two",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-text-two")),
        ));
    }

    if matcher
        .fuzzy_match("TextThree", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.text_three.hex,
            "rgb" => &panther_palette.text_three.rgb.rgb,
            "hsl" => &panther_palette.text_three.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.text_three.hex,
            "rgb" => &tiger_palette.text_three.rgb.rgb,
            "hsl" => &tiger_palette.text_three.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Text Three",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-text-three")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Text Three",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-text-three")),
        ));
    }

    if matcher
        .fuzzy_match("TextFour", &search.search_text)
        .is_some()
    {
        let panther_color = match color_type.as_str() {
            "hex" => &panther_palette.text_four.hex,
            "rgb" => &panther_palette.text_four.rgb.rgb,
            "hsl" => &panther_palette.text_four.hsl.hsl,
            _ => "",
        };

        let tiger_color = match color_type.as_str() {
            "hex" => &tiger_palette.text_four.hex,
            "rgb" => &tiger_palette.text_four.rgb.rgb,
            "hsl" => &tiger_palette.text_four.hsl.hsl,
            _ => "",
        };

        results.push(WLResult::new_text(
            TextResult::new(
                "Panther Text Four",
                Action::new_copy(CopyAction::new(panther_color)),
            )
            .icon(get_icon("panther-text-four")),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Tiger Text Four",
                Action::new_copy(CopyAction::new(tiger_color)),
            )
            .icon(get_icon("tiger-text-four")),
        ));
    }

    send_response(results);
}
