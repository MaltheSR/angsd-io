use std::io;

pub fn read_magic<R>(reader: &mut R, expected: [u8; 8]) -> io::Result<([u8; 8])>
where
    R: io::Read,
{
    let mut magic = [0; 8];

    reader.read_exact(&mut magic)?;

    if magic == expected {
        Ok(magic)
    } else {
        let msg = format!(
            "invalid magic number (expected {:x?}, found {:x?})",
            expected, magic
        );

        Err(io::Error::new(io::ErrorKind::InvalidData, msg))
    }
}

pub fn parse_magic(magic: &[u8; 8]) -> io::Result<String> {
    match std::str::from_utf8(&magic.to_vec()) {
        Ok(s) => {
            let parsed = s.trim_matches(char::from(0));

            Ok(parsed.to_string())
        }
        Err(_) => {
            let msg = format!("unparseable magic number {:x?}", magic);

            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}
