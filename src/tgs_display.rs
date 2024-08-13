use ratatui::{buffer::Buffer, layout::Rect, style::{Style, Stylize}, text::Line, widgets::{canvas::{self, Canvas}, StatefulWidget, Widget}};


pub struct TgsDisplay {
    style: Style,
}

impl TgsDisplay {
    pub fn new() -> Self {
        Self {
            style: Style::default().red().on_black(),
        }
    }
}

impl StatefulWidget for TgsDisplay {
    type State = u8;

    fn render(self, mut area: Rect, buf: &mut Buffer, state: &mut u8) {
        // *state += 1;
        // let text = format!("Frame count: {state}");
        // Line::styled(text, self.style).render(area, buf);

        let s1 = *state & 0b0000_0001 == 0b0000_0001;
        let s2 = *state & 0b0000_0010 == 0b0000_0010;
        let s3 = *state & 0b0000_0100 == 0b0000_0100;
        let s4 = *state & 0b0000_1000 == 0b0000_1000;
        let s5 = *state & 0b0001_0000 == 0b0001_0000;
        let s6 = *state & 0b0010_0000 == 0b0010_0000;
        let s7 = *state & 0b0100_0000 == 0b0100_0000;

        area.width = 9;

        if s1 {
            Line::styled("  █████ ", self.style).render(area, buf);
        } else {
            Line::styled("        ", self.style).render(area, buf);
        }
        area.y += 1;

        if s6 || s2 {
            let mut s = String::with_capacity(9);
            if s6 {
                s.push_str("██");
            } else {
                s.push_str("  ");
            }
            s.push_str("     ");
            if s2 {
                s.push_str("██");
            } else {
                s.push_str("  ");
            }

            Line::styled(&s, self.style).render(area, buf);
            area.y += 1;
            Line::styled(&s, self.style).render(area, buf);
        } else {
            Line::styled("        ", self.style).render(area, buf);
            area.y += 1;
            Line::styled("        ", self.style).render(area, buf);
        }
        area.y += 1;

        if s7 {
            Line::styled("  █████ ", self.style).render(area, buf);
        } else {
            Line::styled("        ", self.style).render(area, buf);
        }
        area.y += 1;

        if s5 || s3 {
            let mut s = String::with_capacity(9);
            if s5 {
                s.push_str("██");
            } else {
                s.push_str("  ");
            }
            s.push_str("     ");
            if s3 {
                s.push_str("██");
            } else {
                s.push_str("  ");
            }

            Line::styled(&s, self.style).render(area, buf);
            area.y += 1;
            Line::styled(&s, self.style).render(area, buf);
        } else {
            Line::styled("        ", self.style).render(area, buf);
            area.y += 1;
            Line::styled("        ", self.style).render(area, buf);
        }
        area.y += 1;

        if s4 {
            Line::styled("  █████ ", self.style).render(area, buf);
        } else {
            Line::styled("        ", self.style).render(area, buf);
        }
    }
}

