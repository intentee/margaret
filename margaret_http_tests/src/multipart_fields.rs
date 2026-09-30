pub const MULTIPART_FIELDS: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\nhello\r\n--X\r\nContent-Disposition: form-data; name=\"subtitle\"\r\n\r\nworld\r\n--X--\r\n";
