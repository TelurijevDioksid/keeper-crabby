use ratatui::{
    buffer::Cell,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Block,
    widgets::{Borders, Widget},
};

use crate::ui::{centered_rect, home_state::Position};

pub struct ScrollView {}

impl ScrollView {
    pub fn check_if_width_out_of_bounds(
        position: &Position,
        buffer_to_render: &Buffer,
        area: Rect,
    ) -> bool {
        let area = centered_rect(area, 97, 94);
        if position.offset_x + area.width - 4 > buffer_to_render.area().width {
            return true;
        }
        false
    }

    pub fn inner_buffer_bounding_box(area: Rect) -> (u16, u16) {
        let area = centered_rect(area, 97, 94);
        (area.width - 4, area.height - 3)
    }

    pub fn render(buffer: &mut Buffer, position: &Position, area: Rect, buffer_to_render: &Buffer) {
        let area = ScrollView::render_borders(buffer, area);
        let area = ScrollView::render_scrollbars(buffer, position, area, buffer_to_render);
        ScrollView::render_view(buffer, position, area, buffer_to_render);
    }

    fn render_borders(buffer: &mut Buffer, area: Rect) -> Rect {
        let b = Block::default().borders(Borders::ALL);

        b.render(area, buffer);

        Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2)
    }

    fn render_scrollbars(
        buffer: &mut Buffer,
        position: &Position,
        area: Rect,
        buffer_to_render: &Buffer,
    ) -> Rect {
        let scrollbar_x_start = area.x;
        let scrollbar_x_end = area.x + area.width;
        let scrollbar_y_start = area.y;
        let scrollbar_y_end = area.y + area.height;
        for i in scrollbar_x_start..scrollbar_x_end - 2 {
            if i == scrollbar_x_start
                || i == scrollbar_x_end - 3
                || i == scrollbar_x_start + 1
                || i == scrollbar_x_end - 4
            {
                buffer[(i, scrollbar_y_end - 1)] = Cell::new("█")
                    .set_style(Style::default().fg(Color::White))
                    .clone();
            } else {
                buffer[(i, scrollbar_y_end - 1)] = Cell::new("━")
                    .set_style(Style::default().fg(Color::White))
                    .clone();
            }
        }
        for i in scrollbar_y_start..scrollbar_y_end - 1 {
            if i == scrollbar_y_start || i == scrollbar_y_end - 2 {
                buffer[(scrollbar_x_end - 2, i)] = Cell::new("██")
                    .set_style(Style::default().fg(Color::White))
                    .clone();
            } else {
                buffer[(scrollbar_x_end - 2, i)] = Cell::new("▕▏")
                    .set_style(Style::default().fg(Color::White))
                    .clone();
            }
        }

        let buffer_to_render_width = buffer_to_render.area().width;
        let buffer_to_render_height = buffer_to_render.area().height;

        let mut scrollbar_x_size = (area.width as f32 - 1.0) / buffer_to_render_width as f32;
        if scrollbar_x_size > 1.0 {
            scrollbar_x_size = 1.0;
        }
        let mut scrollbar_y_size = (area.height as f32 - 2.0) / buffer_to_render_height as f32;
        if scrollbar_y_size > 1.0 {
            scrollbar_y_size = 1.0;
        }

        if scrollbar_x_size < 1.0 {
            let scrollbar_x_position_start = (position.offset_x as f32
                / buffer_to_render_width as f32)
                * (area.width as f32 - 2.0) as f32
                + area.x as f32;
            let scrollbar_x_position_end =
                scrollbar_x_position_start + scrollbar_x_size * (area.width as f32 - 2.0) as f32;

            for i in scrollbar_x_position_start as u16..scrollbar_x_position_end as u16 {
                buffer[(i, scrollbar_y_end - 1)] = Cell::new("▒")
                    .set_style(Style::default().fg(Color::Yellow))
                    .clone();
            }
        }

        if scrollbar_y_size < 1.0 {
            let scrollbar_y_position_start = (position.offset_y as f32
                / buffer_to_render_height as f32)
                * (area.height as f32 - 1.0) as f32
                + area.y as f32;
            let scrollbar_y_position_end =
                scrollbar_y_position_start + scrollbar_y_size * (area.height as f32 - 1.0) as f32;

            for i in scrollbar_y_position_start as u16..scrollbar_y_position_end as u16 {
                buffer[(scrollbar_x_end - 2, i)] = Cell::new("▒▒")
                    .set_style(Style::default().fg(Color::Yellow))
                    .clone();
            }
        }

        let bottom_right_corner = "  ";
        buffer[(scrollbar_x_end - 2, scrollbar_y_end - 1)] = Cell::new(bottom_right_corner)
            .set_style(Style::default().fg(Color::Yellow))
            .clone();

        Rect::new(
            scrollbar_x_start,
            scrollbar_y_start,
            scrollbar_x_end - scrollbar_x_start - 2,
            scrollbar_y_end - scrollbar_y_start - 1,
        )
    }

    fn render_view(
        buffer: &mut Buffer,
        position: &Position,
        area: Rect,
        buffer_to_render: &Buffer,
    ) {
        for i in 0 + area.x..area.width + area.x {
            for j in 0 + area.y..area.height + area.y {
                let cell = buffer_to_render.cell(ratatui::prelude::Position {
                    x: i - area.x + position.offset_x,
                    y: j - area.y + position.offset_y,
                });
                if cell.is_none() || i >= area.width + area.x || j >= area.height + area.y {
                    continue;
                }
                buffer[(i, j)] = buffer_to_render
                    .cell(ratatui::prelude::Position {
                        x: i - area.x + position.offset_x,
                        y: j - area.y + position.offset_y,
                    })
                    .unwrap()
                    .clone();
            }
        }
    }
}
