//! Exact text envelope, not a Markdown parser or a second publication engine.
use crate::{Result, error};

const RESERVED: &str = "<!-- devmeld:entry:";
const BEGIN: &str = "<!-- devmeld:entry:v0:begin -->";
const END: &str = "<!-- devmeld:entry:v0:end -->";
const BOM: &str = "\u{feff}";

fn host(bytes: &[u8]) -> Result<&str> {
    let text = std::str::from_utf8(bytes).map_err(|_| error("instruction host must be UTF-8"))?;
    if text.contains('\0') {
        return Err(error(
            "instruction host contains NUL; UTF-16/32 is unsupported",
        ));
    }
    Ok(text)
}

pub(crate) fn validate_evidence(insertion: &[u8]) -> Result<&'static str> {
    let text = host(insertion)?;
    for newline in ["\r\n", "\n"] {
        if text.starts_with(&format!("{BEGIN}{newline}"))
            && text.ends_with(&format!("{newline}{END}{newline}{newline}"))
            && text.matches(RESERVED).count() == 2
        {
            // Every line in a generated insertion uses the recorded style.
            let normalized = text.replace(newline, "");
            if !normalized.contains(['\r', '\n']) {
                return Ok(newline);
            }
        }
    }
    Err(error("invalid instruction insertion evidence"))
}

pub(crate) struct Prepared {
    pub bytes: Vec<u8>,
    pub insertion: Option<Vec<u8>>,
}

pub(crate) fn prepare(
    before: Option<&[u8]>,
    recorded: Option<&[u8]>,
    body: Option<&str>,
) -> Result<Prepared> {
    let text = host(before.unwrap_or_default())?;
    let (start, end, newline) = if let Some(recorded) = recorded {
        let newline = validate_evidence(recorded)?;
        let insertion = std::str::from_utf8(recorded)?;
        if before.is_none()
            || text.matches(insertion).count() != 1
            || text.matches(RESERVED).count() != 2
        {
            return Err(error(
                "instruction insertion is missing, edited or ambiguous",
            ));
        }
        let start = text
            .find(insertion)
            .ok_or_else(|| error("missing insertion"))?;
        (start, start + insertion.len(), newline)
    } else {
        if text.contains(RESERVED) {
            return Err(error(
                "unowned reserved instruction marker; refusing adoption",
            ));
        }
        if body.is_none() {
            return Err(error("cannot detach an unowned insertion"));
        }
        let newline = match text.as_bytes().iter().position(|b| *b == b'\n') {
            Some(i) if i > 0 && text.as_bytes()[i - 1] == b'\r' => "\r\n",
            _ => "\n",
        };
        let start = if text.starts_with(BOM) { BOM.len() } else { 0 };
        (start, start, newline)
    };
    let insertion = body.map(|body| {
        format!(
            "{BEGIN}{newline}{}{newline}{END}{newline}{newline}",
            body.trim_end_matches('\n').replace('\n', newline)
        )
        .into_bytes()
    });
    if let Some(bytes) = &insertion {
        validate_evidence(bytes)?;
    }
    let mut bytes = text.as_bytes()[..start].to_vec();
    bytes.extend_from_slice(insertion.as_deref().unwrap_or_default());
    bytes.extend_from_slice(&text.as_bytes()[end..]);
    Ok(Prepared { bytes, insertion })
}
