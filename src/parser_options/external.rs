use crate::containers::ExternalStringPtr as StringPtr;
use crate::debug_level;
use crate::source::Decoder;
use crate::token_rewriter::TokenRewriter;

/// Configuration of the parser
#[derive(Debug)]
pub struct ParserOptions {
    /// Name of the buffer. Used in all diagnostic messages
    buffer_name: StringPtr,

    /// Controls which debug information is printed during parsing
    ///
    /// Can be:
    ///
    /// + lib_ruby_parser::debug_level::None
    /// + lib_ruby_parser::debug_level::Parser
    /// + lib_ruby_parser::debug_level::Lexer
    /// + lib_ruby_parser::debug_level::Buffer
    /// + or a combination of them (like `Lexer | Buffer`, these value is just a bitmask)
    debug: debug_level::Type,

    /// Custom decoder that can be used if the source is encoded
    /// in unknown encoding. Only UTF-8 and ASCII-8BIT/BINARY are
    /// supported out of the box.
    ///
    /// # Example
    /// ```rust
    /// use lib_ruby_parser::source::{Decoder, DecoderResult, InputError};
    /// use lib_ruby_parser::{debug_level, Parser, ParserOptions, ParserResult};
    ///
    /// fn decode(encoding: String, input: Vec<u8>) -> DecoderResult {
    ///     if "US-ASCII" == encoding.to_uppercase() {
    ///         // reencode and return Ok(result)
    ///         return DecoderResult::Ok(b"# encoding: us-ascii\ndecoded".to_vec().into());
    ///     }
    ///     DecoderResult::Err(InputError::DecodingError(
    ///         "only us-ascii is supported".into(),
    ///     ))
    /// }
    ///
    /// let decoder = Decoder::new(Box::new(decode));
    /// let options = ParserOptions {
    ///     decoder,
    ///     debug: debug_level::PARSER,
    ///     ..Default::default()
    /// };
    /// let parser = Parser::new(b"# encoding: us-ascii\n3 + 3".to_vec(), options);
    /// let ParserResult { ast, input, .. } = parser.do_parse();
    ///
    /// assert_eq!(
    ///     ast.unwrap().expression().source(&input).unwrap(),
    ///     "decoded".to_string()
    /// )
    /// ```
    decoder: Option<Decoder>,

    /// Optional token rewriter, see TokenRewriter API
    ///
    /// # Example
    /// ```
    /// use lib_ruby_parser::{
    ///     nodes::*,
    ///     token_rewriter::*,
    ///     Bytes, Node, Parser, ParserOptions, ParserResult, Token,
    /// };
    /// fn rewrite_foo_to_bar(mut token: Box<Token>, input: &[u8]) -> TokenRewriterResult {
    ///     // simply rewrite all tokens "foo" to "bar"
    ///     if token.to_string_lossy() == "foo" {
    ///         token.set_token_value(Bytes::new(b"bar".to_vec()));
    ///     }
    ///
    ///     // return token + keep it + keep lexer's state
    ///     TokenRewriterResult {
    ///         rewritten_token: token,
    ///         token_action: RewriteAction::Keep,
    ///         lex_state_action: LexStateAction::Keep,
    ///     }
    /// }
    /// let token_rewriter = TokenRewriter::new(Box::new(rewrite_foo_to_bar));
    /// let options = ParserOptions {
    ///     token_rewriter,
    ///     ..Default::default()
    /// };
    /// let ParserResult { ast, .. } = Parser::new(b"foo = 1".to_vec(), options).do_parse();
    ///
    /// let ast = ast.unwrap();
    ///
    /// let lvar_name = match &*ast {
    ///     Node::Lvasgn(Lvasgn { name, .. }) => name,
    ///     other => panic!("expected lvasgn node, got {:?}", other),
    /// };
    /// assert_eq!(*lvar_name, String::from("bar"));
    /// ```
    token_rewriter: Option<TokenRewriter>,

    /// When set to true Parser records tokens.
    /// When set to false `ParserResult.tokens` is guaranteed to be empty.
    /// If you don't need tokens better set it to false to speed up parsing.
    record_tokens: bool,
}

impl ParserOptions {
    /// Constructs new ParserOptions
    pub fn new(
        buffer_name: StringPtr,
        debug: debug_level::Type,
        decoder: Option<Decoder>,
        token_rewriter: Option<TokenRewriter>,
        record_tokens: bool,
    ) -> Self {
        Self {
            buffer_name,
            debug,
            decoder,
            token_rewriter,
            record_tokens,
        }
    }

    /// Returns `buffer_name` field
    pub fn buffer_name(&self) -> &StringPtr {
        &self.buffer_name
    }
    /// Returns `debug` field
    pub fn debug(&self) -> &debug_level::Type {
        &self.debug
    }
    /// Returns `decoder` field
    pub fn decoder(&self) -> &Option<Decoder> {
        &self.decoder
    }
    /// Returns `token_rewriter` field
    pub fn token_rewriter(&self) -> &Option<TokenRewriter> {
        &self.token_rewriter
    }
    /// Returns `record_tokens` field
    pub fn record_tokens(&self) -> &bool {
        &self.record_tokens
    }
}

use super::InternalParserOptions;
impl From<ParserOptions> for InternalParserOptions {
    fn from(options: ParserOptions) -> Self {
        let ParserOptions {
            buffer_name,
            debug,
            decoder,
            token_rewriter,
            record_tokens,
        } = options;

        Self {
            buffer_name,
            debug,
            decoder,
            token_rewriter,
            record_tokens,
        }
    }
}
