const c = @cImport({
    @cInclude("string.h");
});

pub fn nullTerminateCString(cstr: [*c]const u8) []const u8 {
    const cstr_len = c.strlen(cstr);
    return cstr[0..cstr_len];
}
