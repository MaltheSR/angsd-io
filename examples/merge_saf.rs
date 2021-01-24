use std::{
    env,
    io::{self, Write},
};

use angsd_io::saf::read::{MergedReader, Reader};

fn main() -> io::Result<()> {
    let src: Vec<String> = env::args().skip(1).collect();

    let readers = src
        .iter()
        .map(|x| Reader::from_member_path(x))
        .collect::<io::Result<Vec<Reader<_>>>>()?;

    let mut merge = MergedReader::new(readers);

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    let header = format!("#chrom\tpos\t{}", src.join("\t"));

    writeln!(&mut writer, "{}", header)?;

    for multi_site in merge.iter().map(|x| x.unwrap()) {
        assert!(multi_site
            .iter()
            .all(|site| site.same_location(&multi_site[0])));

        write!(&mut writer, "{}", multi_site[0])?;

        for site in multi_site.iter().skip(1) {
            write!(
                &mut writer,
                "\t{}",
                format!("{}", site).split_terminator('\t').nth(2).unwrap()
            )?;
        }

        writeln!(&mut writer)?;
    }

    Ok(())
}
