// Run the vendored go.mod lexer with the app's normal test suite.
#[path = "../vendor/gpui-kit/crates/shell/src/syntax/gomod.rs"]
mod gomod;

/// The styled text of `text`, in order. Slicing checks that every range sits
/// on character boundaries; unstyled text (punctuation, `=>`) is left out.
fn styled(text: &str) -> Vec<(&str, &'static str)> {
    let highlights = gomod::highlights(text);
    for pair in highlights.windows(2) {
        assert!(pair[0].0.end <= pair[1].0.start, "overlapping: {pair:?}");
    }
    highlights
        .into_iter()
        .map(|(range, style)| (&text[range], style))
        .collect()
}

#[test]
fn every_directive_and_argument_form() {
    let text = include_str!("fixtures/gomod/example.go.mod");
    assert_eq!(
        styled(text),
        [
            (
                "// Every directive and argument form the lexer styles.",
                "comment"
            ),
            ("module", "keyword"),
            ("example.com/app", "string"),
            ("go", "keyword"),
            ("1.24", "constant"),
            ("toolchain", "keyword"),
            ("go1.24.2", "constant"),
            ("godebug", "keyword"),
            ("require", "keyword"),
            ("example.com/lib/v2", "string"),
            ("v2.1.0-beta.1+incompatible", "constant"),
            ("// indirect", "comment"),
            ("golang.org/x/exp", "string"),
            ("v0.0.0-20240506185415-9bf2ced13842", "constant"),
            ("replace", "keyword"),
            ("example.com/a", "string"),
            ("v1.0.0", "constant"),
            ("example.com/b", "string"),
            ("v1.1.0", "constant"),
            ("replace", "keyword"),
            ("example.com/c", "string"),
            ("\"../local dir//c\"", "string"),
            ("exclude", "keyword"),
            ("example.com/bad", "string"),
            ("v1.3.0", "constant"),
            ("retract", "keyword"),
            ("v1.0.0", "constant"),
            ("v1.1.0", "constant"),
            ("// broken releases", "comment"),
            ("tool", "keyword"),
            ("example.com/cmd/x", "string"),
            ("ignore", "keyword"),
            ("./node_modules", "string"),
        ]
    );
}

#[test]
fn broken_lines_do_not_leak() {
    assert!(styled("").is_empty());
    assert_eq!(
        styled("replace a => \"unfinished\ngo 1.24\nrequire (\n"),
        [
            ("replace", "keyword"),
            ("a", "string"),
            ("\"unfinished", "string"),
            ("go", "keyword"),
            ("1.24", "constant"),
            ("require", "keyword"),
        ]
    );
}

#[test]
fn crlf_and_utf8_offsets() {
    assert_eq!(
        styled("module example.com/app // модуль\r\nrequire example.com/x v1.0.0\r\n"),
        [
            ("module", "keyword"),
            ("example.com/app", "string"),
            ("// модуль", "comment"),
            ("require", "keyword"),
            ("example.com/x", "string"),
            ("v1.0.0", "constant"),
        ]
    );
}

#[test]
fn comment_lines_are_not_directives() {
    assert_eq!(
        styled("// require example.com/fake v1.0.0\nmodule example.com/require\n"),
        [
            ("// require example.com/fake v1.0.0", "comment"),
            ("module", "keyword"),
            ("example.com/require", "string"),
        ]
    );
}
