use std::{io::stdout, u16};

use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Result};
use ratatui::{
    backend::CrosstermBackend, buffer::Buffer, crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, layout::Rect, style::{Style, Stylize}, text::Line, widgets::{canvas::Canvas, Paragraph, StatefulWidget, Widget}, Terminal
};
use strum::IntoEnumIterator;
use tgs::{op_code::OpCode, registers::{Register, TgsDisplayValues}, tgs::Tgs, tgs_display::TgsDisplay};
use tracing::info;

fn main() -> Result<()> {
    color_eyre::install()?;
    if let Err(e) = tracing_subscriber::fmt()
        .with_target(true)
        .without_time()
        .with_level(true)
        .try_init()
    {
        return Err(eyre!("Failed to create tracing_subscriber: {e:#?}"));
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut iter = TgsDisplayValues::iter().cycle();
    let mut tgs = Tgs::new();
    let bin = include_bytes!("../assets/hi.bin");
    // let demo1_bin = include_bytes!("../assets/demo1.bin");
    // let demo2_bin = include_bytes!("../assets/demo2.bin");
    let program = OpCode::process_bytes_to_instructions(bin)?;
    // for (i, ins) in x.iter().enumerate() {
    //     info!("{:03}: {}", i + 1, ins);
    // }

    loop {
        terminal.draw(|frame| {
            let mut area = frame.area();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_black(),
                area,
            );

            area.y = 1;
            for (i, v) in tgs.tgs_display().iter_mut().rev().enumerate() {
                area.x = 10*i as u16;
                frame.render_stateful_widget(
                    TgsDisplay::new(),
                    area,
                    v,
                );
            }

            if let Some(instruction) = program.get(tgs.register(Register::PC) as usize) {
                tgs.process_instruction(*instruction);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('a') {
                    *tgs.register_mut_ref(Register::BA) = 1;
                }
            }
        } else {
            *tgs.register_mut_ref(Register::BA) = 0;
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;


    // let bin = include_bytes!("../assets/hi.bin");
    // // let demo1_bin = include_bytes!("../assets/demo1.bin");
    // // let demo2_bin = include_bytes!("../assets/demo2.bin");
    // let x = OpCode::process_bytes_to_instructions(bin)?;
    // for (i, ins) in x.iter().enumerate() {
    //     info!("{:03}: {}", i + 1, ins);
    // }
    //
    // let mut tgs = Tgs::new();
    // tgs.run_program(&x);

    Ok(())
}

#[cfg(test)]
mod test {
    use hex::FromHex;
    use tgs::op_code::OpCode;

    #[test]
    fn test_read() {
        let file = <[u8; 36]>::from_hex(
            "600010100011410000540900611200611300611400611500500000611306611474500000",
        )
        .expect("failed to get bytes");
        let instructions =
            OpCode::process_bytes_to_instructions(&file).expect("failed to process hex to bytes");

        let ins_str = instructions
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let expected_ins_str = r#"MOV %R0, %BA
ADD %R0, %BB
CMP %R0, $0
BNE $9
MOV %D0, $0
MOV %D1, $0
MOV %D2, $0
MOV %D3, $0
BR  $0
MOV %D1, $6
MOV %D2, $116
BR  $0"#;

        assert_eq!(ins_str, expected_ins_str);
    }
}
