use crate::{CatalogError, text};

/// An intentionally bounded lexical reference, not a URL parser or local path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortableLocator(String);

impl PortableLocator {
    pub fn new(value: impl Into<String>) -> Result<Self, CatalogError> {
        let value = value.into();
        text(&value, "locator")?;
        let fail = |reason| CatalogError::InvalidLocator(reason);
        if value.contains(['\\', '?', '#']) || value.chars().any(char::is_whitespace) {
            return Err(fail(
                "backslashes, whitespace, query and fragment are unsupported",
            ));
        }
        let path = if let Some((scheme, remainder)) = value.split_once("://") {
            if !matches!(scheme, "https" | "http" | "ssh" | "git") {
                return Err(fail("unsupported scheme"));
            }
            let (authority, path) = remainder.split_once('/').unwrap_or((remainder, ""));
            // Only plain DNS/IPv4 host spelling and optional numeric port are
            // supported. IPv6/IDNA/credentials need a reviewed parser boundary.
            let (host, port) = authority
                .split_once(':')
                .map_or((authority, None), |(h, p)| (h, Some(p)));
            if host.is_empty()
                || !host.split('.').all(|label| {
                    !label.is_empty()
                        && label
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
                        && !label.starts_with('-')
                        && !label.ends_with('-')
                })
            {
                return Err(fail("invalid or unsupported host"));
            }
            if let Some(port) = port {
                if port.is_empty()
                    || !port.bytes().all(|b| b.is_ascii_digit())
                    || port.parse::<u16>().ok().filter(|n| *n != 0).is_none()
                {
                    return Err(fail("invalid port"));
                }
            }
            path
        } else {
            if value.starts_with(['/', '~']) || value.contains(':') {
                return Err(fail("absolute, home or drive path"));
            }
            value.as_str()
        };
        for (index, segment) in path.split('/').enumerate() {
            let bytes = segment.as_bytes();
            let mut decoded = Vec::with_capacity(bytes.len());
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'%' {
                    let high = bytes.get(i + 1).and_then(|b| (*b as char).to_digit(16));
                    let low = bytes.get(i + 2).and_then(|b| (*b as char).to_digit(16));
                    let (Some(high), Some(low)) = (high, low) else {
                        return Err(fail("malformed escape"));
                    };
                    decoded.push((high * 16 + low) as u8);
                    i += 3;
                } else {
                    decoded.push(bytes[i]);
                    i += 1;
                }
            }
            let decoded = String::from_utf8(decoded).map_err(|_| fail("invalid encoded UTF-8"))?;
            if (index == 0 && !value.contains("://") && decoded.starts_with('~'))
                || matches!(decoded.as_str(), "." | "..")
                || decoded.contains(['%', '/', '\\', ':', '?', '#', '@'])
                || decoded.chars().any(|c| c.is_control() || c.is_whitespace())
            {
                return Err(fail("traversal, reserved or repeatedly encoded segment"));
            }
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceReference {
    id: String,
    locator: PortableLocator,
}
impl SourceReference {
    pub fn new(id: impl Into<String>, locator: PortableLocator) -> Result<Self, CatalogError> {
        let id = id.into();
        text(&id, "source identity")?;
        Ok(Self { id, locator })
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn locator(&self) -> &PortableLocator {
        &self.locator
    }
}
