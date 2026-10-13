pub const MULTIPART_CONTENT_TYPE: &str = "multipart/form-data; boundary=X";

pub const MULTIPART_FIELDS: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\nhello\r\n--X\r\nContent-Disposition: form-data; name=\"subtitle\"\r\n\r\nworld\r\n--X--\r\n";

pub const MULTIPART_FIELDS_AND_FILES: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\nhello\r\n--X\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"face.png\"\r\nContent-Type: image/png\r\n\r\nPNG\r\n--X\r\nContent-Disposition: form-data; name=\"raw\"; filename=\"raw.txt\"\r\n\r\nDATA\r\n--X--\r\n";

pub const MULTIPART_FILES: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"face.png\"\r\nContent-Type: image/png\r\n\r\nPNG\r\n--X\r\nContent-Disposition: form-data; name=\"raw\"; filename=\"raw.txt\"\r\n\r\nDATA\r\n--X--\r\n";
