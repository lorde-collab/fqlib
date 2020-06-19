use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use flate2::bufread::MultiGzDecoder;
use noodles_fastq::Reader;

pub fn open<P>(src: P) -> io::Result<Reader<Box<dyn BufRead>>>
where
    P: AsRef<Path>,
{
    let path = src.as_ref();
    let extenstion = path.extension();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match extenstion.and_then(|ext| ext.to_str()) {
        Some("gz") => {
            let decoder = MultiGzDecoder::new(reader);
            Ok(Reader::new(Box::new(BufReader::new(decoder))))
        }
        _ => Ok(Reader::new(Box::new(reader))),
    }
}
