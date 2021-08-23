#include "blobs.hpp"
#include "impl_blob.hpp"

// Byte
Byte UNPACK_Byte(Byte_BLOB blob)
{
    return blob;
}
Byte_BLOB PACK_Byte(Byte byte)
{
    return byte;
}
IMPL_BLOB(ByteList);

// Ptr
IMPL_BLOB(Ptr);

// MaybePtr
IMPL_BLOB(MaybePtr);

// StringPtr
IMPL_BLOB(StringPtr);

// MaybeStringPtr
IMPL_BLOB(MaybeStringPtr);

// SharedByteList
IMPL_BLOB(SharedByteList);

// SourceLine
IMPL_BLOB(SourceLine);
IMPL_BLOB(SourceLineList);

// Loc
IMPL_BLOB(Loc);

// MaybeLoc
IMPL_BLOB(MaybeLoc);

// Bytes
IMPL_BLOB(Bytes);

// Token
IMPL_BLOB(Token);
IMPL_BLOB(TokenList);

// CommentType
IMPL_BLOB(CommentType);

// Comment
IMPL_BLOB(Comment);
IMPL_BLOB(CommentList);

// MagicCommentKind
IMPL_BLOB(MagicCommentKind);

// MagicComment
IMPL_BLOB(MagicComment);
IMPL_BLOB(MagicCommentList);

// ErrorLevel
IMPL_BLOB(ErrorLevel);

// Diagnostic
IMPL_BLOB(Diagnostic);
IMPL_BLOB(DiagnosticList);

// DiagnosticMessage
IMPL_BLOB(DiagnosticMessage);

// Node
IMPL_BLOB(Node);
IMPL_BLOB(NodeList);
