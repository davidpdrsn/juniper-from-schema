pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

// https://doc.rust-lang.org/reference/keywords.html
static KEYWORDS: [&str; 53] = [
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if",
    "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];
