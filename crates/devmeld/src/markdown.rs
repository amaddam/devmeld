//! Publication presentation only; Markdown syntax is owned by the serializer.
use crate::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

pub(crate) type Inline = Vec<Event<'static>>;

pub(crate) fn text(value: &str) -> Event<'static> {
    Event::Text(single_line(value).into())
}

pub(crate) fn literal(value: &str) -> Event<'static> {
    if value.is_empty() {
        text(value)
    } else {
        Event::Code(single_line(value).into())
    }
}

/// Keep ordinary authored prose readable. If Markdown would reinterpret it,
/// show the whole value literally as code instead of inventing escape rules.
pub(crate) fn authored(value: &str) -> Event<'static> {
    let value = single_line(value);
    let mut parsed_text = String::new();
    let mut paragraphs = 0;
    for event in Parser::new(&value) {
        match event {
            Event::Start(Tag::Paragraph) => paragraphs += 1,
            Event::End(TagEnd::Paragraph) => (),
            Event::Text(part) => parsed_text.push_str(&part),
            _ => return literal(&value),
        }
    }
    if (paragraphs == 1 || value.is_empty()) && parsed_text == value {
        text(&value)
    } else {
        literal(&value)
    }
}

// Authored metadata is a literal single-line value, not a Markdown document.
// Keep the existing control/newline normalization, without escaping syntax here.
fn single_line(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}

pub(crate) fn prose(value: &str) -> Inline {
    let mut events = Vec::new();
    for (index, line) in value.split('\n').enumerate() {
        if index > 0 {
            events.push(Event::SoftBreak);
        }
        events.push(text(line));
    }
    events
}

#[derive(Default)]
pub(crate) struct Document {
    events: Inline,
}

impl Document {
    pub(crate) fn heading(&mut self, level: HeadingLevel, value: &str) {
        self.events.push(Event::Start(Tag::Heading {
            level,
            id: None,
            classes: Vec::new(),
            attrs: Vec::new(),
        }));
        self.events.push(authored(value));
        self.events.push(Event::End(TagEnd::Heading(level)));
    }

    pub(crate) fn paragraph(&mut self, content: impl IntoIterator<Item = Event<'static>>) {
        self.events.push(Event::Start(Tag::Paragraph));
        self.events.extend(content);
        self.events.push(Event::End(TagEnd::Paragraph));
    }

    pub(crate) fn list(&mut self, items: impl IntoIterator<Item = Inline>) {
        self.events.extend(list(items));
    }

    /// Only DevMeld's fixed ownership message may enter an HTML comment.
    pub(crate) fn ownership(&mut self, message: &'static str) {
        self.events.push(Event::Start(Tag::HtmlBlock));
        self.events
            .push(Event::Html(format!("<!-- {message} -->\n").into()));
        self.events.push(Event::End(TagEnd::HtmlBlock));
    }

    pub(crate) fn finish(self) -> Result<String> {
        let mut output = String::new();
        pulldown_cmark_to_cmark::cmark_with_options(
            self.events.iter(),
            &mut output,
            pulldown_cmark_to_cmark::Options {
                list_token: '-',
                ..Default::default()
            },
        )?;
        // The serializer defers inter-block spacing; published files end in one LF.
        output.truncate(output.trim_end_matches('\n').len());
        output.push('\n');
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_values_roundtrip_without_becoming_markdown_structure() {
        for value in [
            "Use test_app.py with HTTP",
            "中文 HTTP / ssh",
            "[ssh] <http>",
            "run **command**",
            "hello &amp; world",
            "1. numbered",
            "---",
            "# heading",
            "[label](https://example.invalid)",
            "![image](x)",
            "`one` and ``two``",
            "  leading and trailing  ",
            "- item",
            "*bold*",
            "a\\b",
            "a\n\tb",
            "<script>run()</script>",
            "[a]: /other",
            "&copy;",
            "   ",
            "",
        ] {
            let mut document = Document::default();
            document.paragraph([authored(value)]);
            let output = document.finish().unwrap();
            let mut actual = String::new();
            for event in Parser::new(&output) {
                match event {
                    Event::Start(Tag::Paragraph) | Event::End(TagEnd::Paragraph) => (),
                    Event::Text(value) | Event::Code(value) => actual.push_str(&value),
                    other => panic!("authored {value:?} became {other:?}: {output}"),
                }
            }
            assert_eq!(actual, single_line(value), "{output}");
        }
    }

    #[test]
    fn keys_and_backtick_literals_use_standard_code_spans() {
        let mut document = Document::default();
        document.list([
            vec![
                literal("use_when"),
                text(": "),
                authored("Read test_app.py"),
            ],
            vec![
                literal("command"),
                text(": "),
                literal("echo `one` and ``two``"),
            ],
        ]);
        let output = document.finish().unwrap();
        assert!(
            output.contains("- `use_when`: Read test_app.py"),
            "{output}"
        );
        assert!(!output.contains("\\_"), "{output}");
        assert!(
            Parser::new(&output)
                .any(|e| matches!(e, Event::Code(v) if v.as_ref() == "echo `one` and ``two``"))
        );
    }
}

pub(crate) fn list(items: impl IntoIterator<Item = Inline>) -> Inline {
    let mut items = items.into_iter().peekable();
    if items.peek().is_none() {
        return Vec::new();
    }
    let mut events = vec![Event::Start(Tag::List(None))];
    for item in items {
        events.push(Event::Start(Tag::Item));
        events.extend(item);
        events.push(Event::End(TagEnd::Item));
    }
    events.push(Event::End(TagEnd::List(false)));
    events
}
