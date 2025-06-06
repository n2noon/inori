use super::artist_select_renderer::render_artist_list;
use super::search_renderer::make_search_box;
use super::status_renderer::render_status;
use super::track_select_renderer::render_track_list;
use super::Theme;
use crate::model::proto::*;
use crate::model::*;
use ratatui::prelude::Constraint::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_search_item<'a>(
    ie: &InfoEntry,
    idx: &[u32],
    theme: &Theme,
) -> Line<'a> {
    let mut out: Vec<Span> = ie
        .to_search_string()
        .chars()
        .map(|c| Span::from(c.to_string()))
        .collect();

    let mut cur = ie.artist.chars().count();
    if let Some(artist_sort) = &ie.artist_sort {
        if *artist_sort != ie.artist {
            let len = artist_sort.chars().count();
            cur += 1; // for spc
            for item in out.iter_mut().take(cur + len + 2).skip(cur) {
                item.style = theme.artist_sort;
            }
            cur += len + 2;
        }
    }
    if let Some(album) = &ie.album {
        let len = album.chars().count();
        out[cur].style = theme.slash_span;
        cur += 1;
        for item in out.iter_mut().skip(cur).take(len) {
            item.style = theme.album;
        }
        cur += len;
    }
    if let Some(_title) = &ie.title {
        out[cur].style = theme.slash_span;
    }
    for (i, item) in out.iter_mut().enumerate() {
        if idx.contains(&u32::try_from(i).unwrap()) {
            item.style = item.style.add_modifier(Modifier::UNDERLINED);
        }
    }
    Line::from(out)
}

pub fn render_global_search(
    model: &mut Model,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
) {
    let layout = Layout::vertical(vec![Max(3), Min(1)])
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(area);

    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::bordered().border_type(BorderType::Rounded),
        area,
    );
    frame.render_widget(
        make_search_box(&model.library.global_search.search.query, true, theme),
        layout[0],
    );
    let list = List::new(
        model
            .library
            .global_search
            .contents()
            .zip(&model.library.global_search.search.cache.indices)
            .map(|(ie, idxs)| render_search_item(ie, idxs, theme)),
    );
    frame.render_stateful_widget(
        list.block(Block::bordered())
            .highlight_style(theme.item_highlight_active),
        layout[1],
        &mut model.library.global_search.results_state,
    );
}

pub fn render(model: &mut Model, frame: &mut Frame, theme: &Theme) {
    let layout = Layout::vertical(vec![Max(4), Min(1)]).split(frame.area());
    let menu_layout =
        Layout::horizontal(vec![Ratio(1, 3), Ratio(2, 3)]).split(layout[1]);
    let header_layout = Layout::horizontal(vec![Ratio(1, 1)]).split(layout[0]);
    let left_panel =
        Layout::vertical(vec![Max(3), Min(1)]).split(menu_layout[0]);
    let right_panel =
        Layout::vertical(vec![Max(3), Min(1)]).split(menu_layout[1]);
    let center_popup_h = Layout::horizontal(vec![
        Percentage(20),
        Percentage(60),
        Percentage(20),
    ])
    .split(frame.area());

    let center_popup_v =
        Layout::vertical(vec![Percentage(20), Percentage(60), Percentage(20)])
            .split(center_popup_h[1]);
    let center_popup = center_popup_v[1];

    if model
        .window_height
        .is_some_and(|i| i != frame.area().height.into())
    {
        model.window_height = Some(frame.area().height.into());
    }

    render_status(model, frame, header_layout[0], theme);

    if model
        .library
        .selected_item()
        .is_some_and(|a| a.search.active)
    {
        frame.render_widget(
            make_search_box(
                &model.library.selected_item().unwrap().search.query,
                matches!(model.state, State::Searching),
                theme,
            ),
            right_panel[0],
        );
        render_track_list(model, frame, right_panel[1], theme);
    } else {
        render_track_list(model, frame, menu_layout[1], theme);
    }

    if model.library.artist_search.active {
        frame.render_widget(
            make_search_box(
                &model.library.artist_search.query,
                matches!(model.state, State::Searching),
                theme,
            ),
            left_panel[0],
        );
        render_artist_list(model, frame, left_panel[1], theme);
    } else {
        render_artist_list(model, frame, menu_layout[0], theme);
    }

    if model.library.global_search.search.active {
        render_global_search(model, frame, center_popup, theme)
    }
}
