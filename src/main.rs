use std::{io::stdout, num::Wrapping};

use color_eyre::{eyre::eyre, Result};
use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, style::Stylize, widgets::Paragraph, Terminal
};
use tgs::{op_code::OpCode, registers::Register, tgs::Tgs, tgs_display::TgsDisplay};

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

    let mut tgs = Tgs::new();
    //let bin = include_bytes!("../assets/hi.bin");
     //let bin = include_bytes!("../assets/demo1.bin");
     let bin = include_bytes!("../assets/demo2.bin");
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
                    &mut v.0,
                );
            }

            if let Some(instruction) = program.get(tgs.register(Register::PC).0 as usize) {
                tgs.process_instruction(*instruction);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('a') {
                    *tgs.register_mut_ref(Register::BA) = Wrapping(1);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('b') {
                    *tgs.register_mut_ref(Register::BB) = Wrapping(1);
                }
            }
        } else {
            *tgs.register_mut_ref(Register::BA) = Wrapping(0);
            *tgs.register_mut_ref(Register::BB) = Wrapping(0);
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

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
