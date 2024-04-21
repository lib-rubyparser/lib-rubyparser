mod fixture;
pub(crate) use fixture::test_file;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

use crate::{
    source::{MagicComment, MagicCommentKind},
    Loc, Parser, ParserOptions, ParserResult,
};

macro_rules! fixture_file {
    ($dir:literal, $fixture:ident) => {
        #[test]
        fn $fixture() {
            let fixture_path = format!("{}/{}", $dir, stringify!($fixture));
            test_file(&fixture_path);
        }
    };
}
pub(crate) use fixture_file;

fn parse(input: &[u8]) -> ParserResult {
    let options = ParserOptions {
        buffer_name: "(eval)".into(),
        record_tokens: false,
        ..Default::default()
    };
    let mut mem = [0; 100];
    let blob = lib_ruby_parser_ast_arena::Blob::from(&mut mem);
    let parser = Parser::new(input, options, &blob);
    parser.do_parse()
}

#[test]
fn test_magic_comment() {
    let fixture = std::fs::read("src/tests/fixtures/magic_comments.rb").unwrap();
    let ParserResult { magic_comments, .. } = parse(&fixture);
    assert_eq!(
        magic_comments,
        vec![
            MagicComment {
                kind: MagicCommentKind::Encoding,
                key_l: Loc { begin: 2, end: 10 },
                value_l: Loc { begin: 12, end: 17 },
            },
            MagicComment {
                kind: MagicCommentKind::FrozenStringLiteral,
                key_l: Loc { begin: 20, end: 41 },
                value_l: Loc { begin: 43, end: 47 },
            },
            MagicComment {
                kind: MagicCommentKind::Encoding,
                key_l: Loc { begin: 50, end: 56 },
                value_l: Loc { begin: 58, end: 63 },
            },
            MagicComment {
                kind: MagicCommentKind::ShareableConstantValue,
                key_l: Loc { begin: 66, end: 90 },
                value_l: Loc { begin: 92, end: 99 },
            },
            MagicComment {
                kind: MagicCommentKind::WarnIndent,
                key_l: Loc {
                    begin: 102,
                    end: 113
                },
                value_l: Loc {
                    begin: 115,
                    end: 119
                },
            },
        ]
    );
}
