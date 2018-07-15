/// File is structured as:
/// - Schema version/hash (u64)
/// - number of entries N (u32) <= capacity
/// - per entry:
///   - 2 if empty, 1 if present (u8), anything else means corrupted
///   - the entry (known length), or garbage if empty
/// - <garbage to fill filesize>
///
/// E.g. if rows are 4+8 bytes and we have entry 0 and 2 (1 deleted):
/// - from byte 0: <version hash that says row is 12 bytes>
/// - from byte 8: 3
/// - from byte 12: 1
/// - from byte 13: entry#0
/// - from byte 25: 2
/// - from byte 26: <garbage>
/// - from byte 38: 1
/// - from byte 39: entry#2
/// - from byte 51: <garbage to fill filesize>

pub struct RowFile {}

impl RowFile {
    pub fn write_all() {}

    pub fn read_all() {}
}
