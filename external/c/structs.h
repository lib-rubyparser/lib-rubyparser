#ifndef LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
#define LIB_RUBY_PARSER_C_BINDINGS_STRUCTS

#include <stddef.h>
#include <stdbool.h>
#include "declare_list.h"

// Byte
typedef uint8_t Byte;
DECLARE_LIST_OF(uint8_t, ByteList);
void drop_byte(Byte *);
void drop_byte_list(ByteList *);

// Ptr
typedef void *Ptr;

// MaybePtr
typedef void *MaybePtr;

// StringPtr
typedef struct StringPtr
{
    uint8_t *ptr;
    uint64_t len;
} StringPtr;
void drop_string_ptr(StringPtr *);

// MaybeStringPtr
typedef struct MaybeStringPtr
{
    uint8_t *ptr;
    uint64_t len;
} MaybeStringPtr;
void drop_maybe_string_ptr(MaybeStringPtr *maybe_string_ptr);

// SharedByteList
typedef struct SharedByteList
{
    const uint8_t *ptr;
    uint64_t len;
} SharedByteList;

// SourceLine
typedef struct SourceLine
{
    uint64_t start;
    uint64_t end;
    bool ends_with_eof;
} SourceLine;
DECLARE_LIST_OF(SourceLine, SourceLineList);
void drop_source_line_list(SourceLineList *source_line_list);

// Loc
typedef struct Loc
{
    uint64_t begin;
    uint64_t end;
} Loc;
void drop_loc(Loc *loc);

// MaybeLoc
typedef struct MaybeLoc
{
    enum
    {
        MAYBE_LOC_SOME,
        MAYBE_LOC_NONE,
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        Loc loc;
    } as;
} MaybeLoc;
void drop_maybe_loc(MaybeLoc *maybe_loc);

// Bytes
typedef struct Bytes
{
    ByteList raw;
} Bytes;
void drop_bytes(Bytes *bytes);

// Token
typedef struct Token
{
    uint32_t token_type;
    Bytes token_value;
    Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;
} Token;
DECLARE_LIST_OF(Token, TokenList);
void drop_token(Token *);
void drop_token_list(TokenList *);

// CommentType
typedef enum CommentType
{
    DOCUMENT,
    INLINE,
    UNKNOWN,
} CommentType;

// Comment
typedef struct Comment
{
    Loc location;
    CommentType kind;
} Comment;
DECLARE_LIST_OF(Comment, CommentList);
void drop_comment_list(CommentList *);

// MagicCommentKind
typedef enum MagicCommentKind
{
    MAGIC_COMMENT_KIND_ENCODING,
    MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL,
    MAGIC_COMMENT_KIND_WARN_INDENT,
    MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE,
} MagicCommentKind;

// MagicComment
typedef struct MagicComment
{
    MagicCommentKind kind;
    Loc key_l;
    Loc value_l;
} MagicComment;
DECLARE_LIST_OF(MagicComment, MagicCommentList);
void drop_magic_comment_list(MagicCommentList *);

// ErrorLevel
typedef enum ErrorLevel
{
    WARNING,
    ERROR
} ErrorLevel;

// DiagnosticMessage
#include "messages.h"
void drop_diagnostic_list(DiagnosticList *);

// Node
#include "nodes.h"

// InputError
typedef struct InputError
{
    enum
    {
        UNSUPPORTED_ENCODING,
        DECODING_ERROR
    } tag;

    union
    {
        StringPtr unsupported_encoding;
        StringPtr decoding_error;
    } as;
} InputError;
void drop_input_error(InputError *);

// DecoderResult
typedef struct DecoderResult
{
    enum
    {
        DECODE_OK,
        DECODE_ERR
    } tag;

    union
    {
        ByteList ok;
        InputError err;
    } as;
} DecoderResult;
void drop_decoder_result(DecoderResult *);

// Decoder
typedef DecoderResult (*dummy_decoder_t)(void);
typedef struct Decoder
{
    // Here for tests we use a dummy fn that (when called) blindly returns what's configured
    dummy_decoder_t f;
} Decoder;

// TokenRewriter
typedef enum RewriteAction
{
    REWRITE_ACTION_DROP,
    REWRITE_ACTION_KEEP
} RewriteAction;
typedef struct LexStateAction
{
    enum
    {
        LEX_STATE_SET,
        LEX_STATE_KEEP
    } tag;
    union
    {
        int32_t set;
    } as;
} LexStateAction;
typedef struct TokenRewriterResult
{
    Token *rewritten_token;
    RewriteAction token_action;
    LexStateAction lex_state_action;
} TokenRewriterResult;
void drop_token_rewriter_result(TokenRewriterResult *);
typedef Token *(*build_new_token_t)(void);
typedef TokenRewriterResult (*rewrite_token_t)(Token *, build_new_token_t);
typedef struct TokenRewriter
{
    // Here for tests we use a dummy fn that (when called) blindly returns what's configured
    rewrite_token_t rewrite_f;
    build_new_token_t build_new_token_f;
} TokenRewriter;
// Test APIS
TokenRewriter __keep_token_rewriter(build_new_token_t build_new_token_f);
TokenRewriter __drop_token_rewriter(build_new_token_t build_new_token_f);
TokenRewriter __rewriter_token_rewriter(build_new_token_t build_new_token_f);

// ParserOptions
typedef struct MaybeDecoder
{
    enum
    {
        MAYBE_DECODER_SOME,
        MAYBE_DECODER_NONE
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        Decoder decoder;
    } as;
} MaybeDecoder;
typedef struct MaybeTokenRewriter
{
    enum
    {
        MAYBE_TOKEN_REWRITER_SOME,
        MAYBE_TOKEN_REWRITER_NONE
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        TokenRewriter token_rewriter;
    } as;
} MaybeTokenRewriter;
typedef struct ParserOptions
{
    StringPtr buffer_name;
    uint8_t debug;
    MaybeDecoder decoder;
    MaybeTokenRewriter token_rewriter;
    bool record_tokens;
} ParserOptions;
void drop_parser_options(ParserOptions *);

// DecodedInput
typedef struct DecodedInput
{
    StringPtr name;
    SourceLineList lines;
    ByteList bytes;
} DecodedInput;
void drop_decoded_input(DecodedInput *);

// ParserResult
typedef struct ParserResult
{
    Node *ast;
    TokenList tokens;
    DiagnosticList diagnostics;
    CommentList comments;
    MagicCommentList magic_comments;
    DecodedInput input;
} ParserResult;
void drop_parser_result(ParserResult *);

#endif // LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
