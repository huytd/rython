use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::StatefulWidget;
use ratatui_textarea::TextArea;


/// Token types for Python syntax highlighting.
#[derive(Clone, Copy)]
pub enum TokenType {
    Keyword,
    BuiltIn,
    String,
    Comment,
    Number,
    Decorator,
    FunctionName,
    Operator,
    Parenthesis,
    Bracket,
}

impl TokenType {
    pub fn style(&self) -> Style {
        match self {
            TokenType::Keyword => Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            TokenType::BuiltIn => Style::new().fg(Color::Magenta),
            TokenType::String => Style::new().fg(Color::Yellow),
            TokenType::Comment => Style::new().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            TokenType::Number => Style::new().fg(Color::Green),
            TokenType::Decorator => Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            TokenType::FunctionName => Style::new().fg(Color::Cyan),
            TokenType::Operator => Style::new().fg(Color::White),
            TokenType::Parenthesis => Style::new().fg(Color::Gray),
            TokenType::Bracket => Style::new().fg(Color::Gray),
        }
    }
}

/// Python keywords to highlight.
const KEYWORDS: &[&str] = &[
    "and", "as", "assert", "break", "class", "continue", "def", "del", "elif",
    "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
    "try", "while", "with", "yield", "True", "False", "None",
];

/// Common Python built-in functions and types.
const BUILTINS: &[&str] = &[
    "abs", "all", "any", "bin", "bool", "bytearray", "bytes", "callable",
    "chr", "classmethod", "compile", "complex", "dict", "dir", "divmod",
    "enumerate", "eval", "filter", "float", "format", "frozenset", "getattr",
    "globals", "hasattr", "hash", "hex", "id", "input", "int", "isinstance",
    "issubclass", "iter", "len", "list", "locals", "map", "max", "min",
    "next", "object", "oct", "open", "ord", "pow", "print", "property",
    "range", "repr", "reversed", "round", "set", "setattr", "slice", "sorted",
    "staticmethod", "str", "sum", "super", "tuple", "type", "vars", "zip",
    "screen", "time", "os", "sys",
];

/// Check if a character is valid in a Python identifier.
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Tokenize a single line of Python code into styled spans.
pub fn highlight_line(line: &str) -> Vec<Span<'_>> {
    let mut spans = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    while i < len {
        // Skip whitespace (preserve it)
        if chars[i].is_whitespace() {
            spans.push(Span::raw(chars[i].to_string()));
            i += 1;
            continue;
        }

        // Comments
        if chars[i] == '#' {
            let rest: String = chars[i..].iter().collect();
            spans.push(Span::styled(rest, TokenType::Comment.style()));
            break;
        }

        // Decorators
        if chars[i] == '@' && (i == 0 || chars[i - 1].is_whitespace()) {
            let mut j = i + 1;
            while j < len && is_ident_char(chars[j]) {
                j += 1;
            }
            if j > i + 1 {
                let decorator: String = chars[i..j].iter().collect();
                spans.push(Span::styled(decorator, TokenType::Decorator.style()));
                i = j;
                continue;
            }
        }

        // Strings (single or double quotes, triple quotes on single line)
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            let mut j = i + 1;
            let triple = i + 2 < len && chars[i + 1] == quote && chars[i + 2] == quote;
            if triple {
                j += 2;
                while j + 2 < len {
                    if chars[j] == quote && chars[j + 1] == quote && chars[j + 2] == quote {
                        j += 3;
                        break;
                    }
                    if chars[j] == '\\' && j + 1 < len {
                        j += 2;
                    } else {
                        j += 1;
                    }
                }
            } else {
                while j < len && chars[j] != quote {
                    if chars[j] == '\\' && j + 1 < len {
                        j += 2;
                    } else {
                        j += 1;
                    }
                }
            }
            if j <= len {
                j += 1;
            }
            let string: String = chars[i..j.min(len)].iter().collect();
            spans.push(Span::styled(string, TokenType::String.style()));
            i = j;
            continue;
        }

        // f-strings
        if (chars[i] == 'f' || chars[i] == 'F') && i + 1 < len && (chars[i + 1] == '"' || chars[i + 1] == '\'') {
            let quote = chars[i + 1];
            let mut j = i + 2;
            while j < len && chars[j] != quote {
                if chars[j] == '{' && j + 1 < len && chars[j + 1] != '}' {
                    let expr_start = j;
                    j += 1;
                    while j < len && chars[j] != '}' {
                        if chars[j] == '\\' && j + 1 < len {
                            j += 2;
                        } else {
                            j += 1;
                        }
                    }
                    let expr: String = chars[expr_start + 1..j].iter().collect();
                    spans.push(Span::styled("{".to_string(), TokenType::Bracket.style()));
                    for span in highlight_line(&expr) {
                        spans.push(Span::styled(span.content.into_owned(), span.style));
                    }
                    if j < len {
                        spans.push(Span::styled("}".to_string(), TokenType::Bracket.style()));
                        j += 1;
                    }
                } else if chars[j] == '\\' && j + 1 < len {
                    j += 2;
                } else {
                    j += 1;
                }
            }
            if j <= len {
                j += 1;
            }
            let prefix_quote: String = chars[i..i + 2].iter().collect();
            spans.push(Span::styled(prefix_quote, TokenType::String.style()));
            let j_clamped = j.min(len);
            if j_clamped > i + 2 {
                let closing: String = chars[j_clamped - 1..j_clamped].iter().collect();
                spans.push(Span::styled(closing, TokenType::String.style()));
            }
            i = j;
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() || (chars[i] == '.' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let mut j = i;
            if chars[i] == '0' && i + 1 < len && "xXbBoO".contains(chars[i + 1]) {
                j += 2;
                while j < len && chars[j].is_ascii_hexdigit() {
                    j += 1;
                }
            } else {
                while j < len && (chars[j].is_ascii_digit() || chars[j] == '.') {
                    j += 1;
                }
                if j < len && (chars[j] == 'e' || chars[j] == 'E') {
                    j += 1;
                    if j < len && (chars[j] == '+' || chars[j] == '-') {
                        j += 1;
                    }
                    while j < len && chars[j].is_ascii_digit() {
                        j += 1;
                    }
                }
            }
            if j < len && (chars[j] == 'j' || chars[j] == 'J') {
                j += 1;
            }
            let number: String = chars[i..j].iter().collect();
            spans.push(Span::styled(number, TokenType::Number.style()));
            i = j;
            continue;
        }

        // Identifiers
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' {
            let mut j = i + 1;
            while j < len && is_ident_char(chars[j]) {
                j += 1;
            }
            let ident: String = chars[i..j].iter().collect();
            let rest_trimmed: String = chars[j..].iter().skip_while(|c| c.is_whitespace()).collect();
            let is_call = rest_trimmed.starts_with('(');

            if KEYWORDS.binary_search(&ident.as_str()).is_ok() {
                spans.push(Span::styled(ident, TokenType::Keyword.style()));
            } else if BUILTINS.binary_search(&ident.as_str()).is_ok() {
                spans.push(Span::styled(ident, TokenType::BuiltIn.style()));
            } else if is_call {
                spans.push(Span::styled(ident, TokenType::FunctionName.style()));
            } else {
                spans.push(Span::raw(ident));
            }
            i = j;
            continue;
        }

        // Operators
        let op_chars = "+-*/%<>!=&|^~";
        if op_chars.contains(chars[i]) {
            let mut j = i + 1;
            while j < len && op_chars.contains(chars[j]) && (j - i) < 2 {
                j += 1;
            }
            let op: String = chars[i..j].iter().collect();
            spans.push(Span::styled(op, TokenType::Operator.style()));
            i = j;
            continue;
        }

        // Parentheses and brackets
        if chars[i] == '(' || chars[i] == ')' {
            spans.push(Span::styled(chars[i].to_string(), TokenType::Parenthesis.style()));
            i += 1;
            continue;
        }
        if chars[i] == '[' || chars[i] == ']' {
            spans.push(Span::styled(chars[i].to_string(), TokenType::Bracket.style()));
            i += 1;
            continue;
        }

        // Other punctuation
        if "+-*/%<>!=&|^~()[]{}:;,.".contains(chars[i]) {
            spans.push(Span::raw(chars[i].to_string()));
            i += 1;
            continue;
        }

        // Default
        spans.push(Span::raw(chars[i].to_string()));
        i += 1;
    }

    spans
}

/// Highlight an entire code block, returning a vector of styled lines.
#[allow(dead_code)]
pub fn highlight(code: &str) -> Vec<Line<'_>> {
    code.lines()
        .map(|line| Line::from(highlight_line(line)))
        .collect()
}

/// A widget that renders textarea content with Python syntax highlighting.
pub struct CodeEditor<'a> {
    pub textarea: &'a TextArea<'a>,
}

impl StatefulWidget for CodeEditor<'_> {
    type State = ();

    fn render(self, area: Rect, buffer: &mut Buffer, _state: &mut Self::State) {
        let lines: Vec<&str> = self.textarea.lines().iter().map(|l| l.as_str()).collect();
        let cursor = self.textarea.cursor();
        let cursor_row = cursor.0;
        let cursor_col = cursor.1;

        let total_lines = lines.len().max(1);
        let visible_height = area.height as usize;
        let visible_width = area.width as usize;

        // Calculate scroll offset: keep the cursor centered vertically,
        // but clamp so it never leaves the viewport.
        // This single formula is continuous — no jumps at band boundaries.
        let mut scroll_offset = cursor_row.saturating_sub(visible_height / 2);
        let max_offset = total_lines.saturating_sub(visible_height);
        scroll_offset = scroll_offset.min(max_offset);
        // If the cursor is above the top of the viewport, reduce offset.
        if cursor_row < scroll_offset {
            scroll_offset = cursor_row;
        }
        // If the cursor is below the bottom of the viewport, increase offset.
        let visual_row = cursor_row - scroll_offset;
        if visual_row >= visible_height {
            scroll_offset += visual_row - visible_height + 1;
        }

        let cursor_bg = Color::Rgb(50, 50, 50);

        for (vi, line_idx) in (scroll_offset..total_lines.min(scroll_offset + visible_height)).enumerate() {
            let row = vi as u16;
            if row >= area.height {
                break;
            }

            let y = area.y + row;
            let line_text = if line_idx < lines.len() {
                lines[line_idx]
            } else {
                ""
            };

            let styled_spans = highlight_line(line_text);
            let is_cursor_line = line_idx == cursor_row;

            // Flatten spans into individual characters with proper styling
            let mut cells: Vec<(String, Style)> = Vec::new();
            for span in &styled_spans {
                for ch in span.content.chars() {
                    cells.push((ch.to_string(), span.style));
                }
            }

            // Render character by character
            for (ci, (ch_str, style)) in cells.iter().take(visible_width).enumerate() {
                let x = area.x + ci as u16;
                let cell = &mut buffer[(x, y)];
                if is_cursor_line {
                    let merged = style.patch(Style::new().bg(cursor_bg));
                    cell.set_style(merged);
                } else {
                    cell.set_style(*style);
                }
                cell.set_symbol(ch_str);
            }

            // Fill remaining cells to clear old content
            let rendered_len = cells.len().min(visible_width);
            for ci in rendered_len..visible_width {
                let x = area.x + ci as u16;
                let cell = &mut buffer[(x, y)];
                if is_cursor_line {
                    cell.set_bg(cursor_bg);
                }
                cell.reset();
            }

            // Draw cursor at the current column, clamped within the visible area.
            // Clamp to visible_width - 1 so we never index past the right edge of the buffer.
            if is_cursor_line {
                let max_col = (visible_width - 1).min(line_text.chars().count());
                let cx = area.x + cursor_col.min(max_col) as u16;
                let cell = &mut buffer[(cx, y)];
                if cursor_col >= line_text.chars().count() {
                    cell.set_char(' ');
                }
                // Yellow background + black text for the cursor
                cell.set_style(Style::new().bg(Color::Yellow).fg(Color::Black));
            }
        }

        // Clear any leftover lines below our content
        let rendered_lines = (total_lines - scroll_offset).min(visible_height);
        for vi in rendered_lines..visible_height {
            let y = area.y + vi as u16;
            for x in area.x..area.x + area.width {
                let cell = &mut buffer[(x, y)];
                cell.reset();
            }
        }
    }
}
