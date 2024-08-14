
use ratatui::{buffer::Buffer, layout::Rect, style::{Style, Stylize}, text::{Line, Span}, widgets::{StatefulWidget, Widget}};


pub struct TgsDisplay {
    led_on_style: Style,
    led_off_style: Style,
}

impl TgsDisplay {
    pub fn new() -> Self {
        Self {
            led_on_style: Style::default().red().on_black(),
            led_off_style: Style::default().fg(ratatui::style::Color::DarkGray).on_black(),
        }
    }

    fn led_style_if(&self, on: bool) -> Style {
        if on {
            self.led_on_style
        } else {
            self.led_off_style
        }
    }
}

impl Default for TgsDisplay {
    fn default() -> Self {
        Self::new()
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

        Line::styled("  █████ ", self.led_style_if(s1)).render(area, buf);
        area.y += 1;

        let p1 = Span::styled("██     ", self.led_style_if(s6));
        let p2 = Span::styled("██", self.led_style_if(s2));
        Line::default().spans([p1.clone(), p2.clone()]).render(area, buf);
        area.y += 1;
        Line::default().spans([p1, p2]).render(area, buf);
        area.y += 1;

        Line::styled("  █████ ", self.led_style_if(s7)).render(area, buf);
        area.y += 1;

        let p1 = Span::styled("██     ", self.led_style_if(s5));
        let p2 = Span::styled("██", self.led_style_if(s3));
        Line::default().spans([p1.clone(), p2.clone()]).render(area, buf);
        area.y += 1;
        Line::default().spans([p1, p2]).render(area, buf);
        area.y += 1;

        Line::styled("  █████ ", self.led_style_if(s4)).render(area, buf);
    }
}

