use std::{
    env,
    io::{self, Write},
};

use angsd_io::saf::read::Reader;

fn main() -> io::Result<()> {
    let src = env::args().nth(1).expect("missing path to SAF member file");

    let mut reader = Reader::from_member_path(&src)?;

    let header = reader.read_header()?;
    assert_eq!(header, String::from("safv3"));

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    writeln!(&mut writer, "{}", reader.index())?;

    let header = format!("#chrom\tpos\t{}", src);

    writeln!(&mut writer, "{}", header)?;

    for site in reader.sites() {
        writeln!(&mut writer, "{}", site?)?;
    }

    Ok(())
}
