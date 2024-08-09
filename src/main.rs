use color_eyre::{eyre::eyre, Result};
use tgs::op_code::OpCode;
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

    use hex::FromHex;
    let file = <[u8; 36]>::from_hex(
        "600010100011410000540900611200611300611400611500500000611306611474500000",
    )
    .expect("failed to get bytes");
    let x = OpCode::process_bytes_to_instructions(&file)?;
    for (i, ins) in x.iter().enumerate() {
        info!("{:03}: {}", i + 1, ins);
    }

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
