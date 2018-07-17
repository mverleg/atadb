/// File is structured as:
/// - number of entries N (u32) <= capacity
/// - for each entry (N times) except the first, ordered by appearance:
///   - offset of the entry (u32)
/// - offset of the last empty spot
/// - <garbage until capacity reached>
/// - for each entry (N times), starting at offset 0
///   - 2 if empty, 1 if present (u8), anything else means corrupted
///   - the entry at the specified offset (until the next offset), or garbage if empty
/// - <garbage to fill filesize>
///
/// E.g. for a text "hi", a deleted four-byte entry and "你好" (6 bytes in utf8), and capacity of 16 entries:
/// - from byte 0: 3
/// - from byte 4: 3
/// - from byte 8: 8 (3+5)
/// - from byte 12: 15 (8+7)
/// - from byte 16: <13*4 bytes garbage to reach capacity>
/// - from byte 68+0: 1
/// - from byte 68+1: "hi"
/// - from byte 68+3: 2
/// - from byte 68+4: <4 bytes of garbage>
/// - from byte 68+8: 1
/// - from byte 68+9: "你好"
/// - from byte 68+15: <garbage to fill filesize>
// TODO @mverleg: it may be necessary at some point to add back-references for cleanup

pub struct InternFile {}
