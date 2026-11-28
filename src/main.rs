use std::io::{self, BufRead, Write};
mod rfcomm;

#[derive(Clone, Copy, PartialEq)]
enum Profile { Basic, Focus2007 }

struct Emulator { profile: Profile, headers: bool }
impl Emulator {
    fn new(profile: Profile) -> Self { Self { profile, headers: false } }
    fn execute(&mut self, command: &str) -> String {
        let cmd = command.trim().to_ascii_uppercase();
        let focus = self.profile == Profile::Focus2007;
        let answer = match cmd.as_str() {
            "ATZ" => { self.headers = false; if focus { "ELM327 v2.3" } else { "ELM327 v1.5" } },
            "ATI" => if focus { "ELM327 v2.3" } else { "ELM327 v1.5" },
            "ATE0" | "ATL0" | "ATS1" | "ATSP0" => "OK",
            "ATH0" => { self.headers = false; "OK" },
            "ATH1" => { self.headers = true; "OK" },
            "ATDP" if focus => "ISO 15765-4 (CAN 11/500) SIMULATED",
            "ATDPN" if focus => "A6",
            "0100" if focus => "41 00 98 3B 00 11",
            "0120" if focus => "41 20 A0 00 00 00",
            "0140" | "0149" | "014A" | "014B" | "015A" if focus => "NO DATA",
            _ => "?",
        };
        // Header and protocol are synthetic: the original capture had ATH0.
        let payload = if self.headers && answer.starts_with("41 ") {
            format!("7E8 {:02X} {}", answer.split_whitespace().count(), answer)
        } else { answer.to_owned() };
        format!("{payload}\r\r>")
    }
}

fn serve(mut reader: impl BufRead, mut writer: impl Write, profile: Profile) -> io::Result<()> {
    let mut emulator = Emulator::new(profile);
    loop {
        let mut command = Vec::new();
        if reader.read_until(b'\r', &mut command)? == 0 { break; }
        let response = emulator.execute(&String::from_utf8_lossy(&command));
        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut profile = Profile::Basic;
    let mut bluetooth = false;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--rfcomm" => bluetooth = true,
            "--profile" => match it.next().map(String::as_str) {
                Some("focus-2007") => profile = Profile::Focus2007,
                Some("basic") => profile = Profile::Basic,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown profile")),
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown argument")),
        }
    }
    if bluetooth { rfcomm::accept_one_connection(profile) }
    else { serve(io::stdin().lock(), io::stdout().lock(), profile) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn ati_returns_adapter_identification() {
        assert_eq!(Emulator::new(Profile::Basic).execute("ATI"), "ELM327 v1.5\r\r>");
    }
    #[test] fn captured_bitmaps_and_header_mode() {
        let mut e = Emulator::new(Profile::Focus2007);
        assert_eq!(e.execute("0100"), "41 00 98 3B 00 11\r\r>");
        assert_eq!(e.execute("0120"), "41 20 A0 00 00 00\r\r>");
        assert_eq!(e.execute("0149"), "NO DATA\r\r>");
        e.execute("ATH1");
        assert_eq!(e.execute("0100"), "7E8 06 41 00 98 3B 00 11\r\r>");
        e.execute("ATZ");
        assert_eq!(e.execute("0100"), "41 00 98 3B 00 11\r\r>");
    }
    #[test] fn stream_replay() {
        let mut output = Vec::new();
        serve(&b"0100\r0120\r"[..], &mut output, Profile::Focus2007).unwrap();
        assert_eq!(output, b"41 00 98 3B 00 11\r\r>41 20 A0 00 00 00\r\r>");
    }
}
