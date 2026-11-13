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
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();

    loop {
        let mut command = Vec::new();

        let bytes_read = reader.read_until(b'\r', &mut command)?;

        if bytes_read == 0 {
            break;
        }

        if command.last() == Some(&b'\r') {
            command.pop();
        }

        let command = String::from_utf8_lossy(&command);
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

