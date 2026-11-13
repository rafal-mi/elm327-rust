use std::io::{self, BufRead, Write};

fn execute(command: &str) -> &'static str {
    match command {
        "ATI" => "ELM327 v1.5\r\r>",
        "ATE0" => "OK\r\r>",
        "ATZ" => "ELM327 v1.5\r\r>",
        _ => "?\r\r>",
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let command = line?;
        let response = execute(command.trim());

        stdout.write_all(response.as_bytes())?;
        stdout.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ati_returns_adapter_identification() {
        assert_eq!(execute("ATI"), "ELM327 v1.5\r\r>");
    }
}

