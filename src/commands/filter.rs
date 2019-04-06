use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::ArgMatches;
use log::info;
use noodles::formats::fastq;

fn copy_filtered<R, W>(
    mut reader: fastq::Reader<R>,
    names: &HashSet<Vec<u8>>,
    mut writer: fastq::Writer<W>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut record = fastq::Record::default();

    loop {
        let bytes_read = reader.read_record(&mut record)?;

        if bytes_read == 0 {
            break;
        }

        if names.contains(record.name()) {
            writer.write_record(&record)?;
        }
    }

    Ok(())
}

fn read_names<R>(reader: R) -> io::Result<HashSet<Vec<u8>>>
where
    R: BufRead,
{
    let mut names = HashSet::new();

    for result in reader.lines() {
        let line = result?;
        names.insert(line.into_bytes());
    }

    Ok(names)
}

pub fn filter(matches: &ArgMatches) {
    let src = matches.value_of("src").unwrap();
    let names_src = matches.value_of("names").unwrap();

    info!("fq-filter start");

    info!("reading names");

    let file = File::open(names_src).unwrap();
    let reader = BufReader::new(file);
    let names = read_names(reader).unwrap();

    info!("read {} names", names.len());

    let stdout = io::stdout();
    let handle = stdout.lock();
    let buf = BufWriter::new(handle);
    let writer = fastq::Writer::new(buf);

    info!("filtering fastq");

    let reader = fastq::reader::open(src).unwrap();
    copy_filtered(reader, &names, writer).unwrap();

    info!("fq-filter end");
}

#[cfg(test)]
mod tests {
    use super::*;

    use noodles::formats::fastq;

    #[test]
    fn test_copy_filtered() {
        let names = [b"@fqlib:2/1".to_vec()].iter().cloned().collect();

        let data = "\
@fqlib:1/1\nAGCT\n+\nabcd
@fqlib:2/1\nTCGA\n+\ndcba
@fqlib:3/1\nGCCA\n+\ngcca
";

        let reader = fastq::Reader::new(data.as_bytes());

        let mut buf = Vec::new();
        let writer = fastq::Writer::new(&mut buf);

        copy_filtered(reader, &names, writer).unwrap();

        let expected = b"@fqlib:2/1\nTCGA\n+\ndcba\n";
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_read_names() {
        let data = "@fqlib:1/1\n@fqlib:2/1\n@fqlib:3/1\n";

        let names = read_names(data.as_bytes()).unwrap();

        assert_eq!(names.len(), 3);
        assert!(names.contains("@fqlib:1/1".as_bytes()));
        assert!(names.contains("@fqlib:2/1".as_bytes()));
        assert!(names.contains("@fqlib:3/1".as_bytes()));
    }
}
