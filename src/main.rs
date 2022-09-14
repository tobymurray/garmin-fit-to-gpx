use bigdecimal::{BigDecimal, ToPrimitive};
use core::panic;
use fitparser::profile::MesgNum;
use geo_types::Point;
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    let paths = read_test_data();
    println!("There are {} files", paths.len());

    for path in paths {
        parse_file(path);
    }
}

fn parse_file(path: PathBuf) {
    println!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );
    let path_clone = path.clone();
    let mut fp = File::open(path).unwrap();
    let parser = fitparser::from_reader(&mut fp).unwrap();

    let mut track: Track = Track::new();

    let mut segment = TrackSegment::new();

    for data in parser {
        match data.kind() {
            MesgNum::UnknownVariant(_) => continue,
            MesgNum::Record => {
                let mut latitude: Option<i32> = None;
                let mut longitude: Option<i32> = None;
                // writeln!(&mut file, "{:?}", data).unwrap();
                for field in data.fields() {
                    match field.name() {
                        "position_lat" => match field.value() {
                            fitparser::Value::SInt32(v) => {
                                latitude = Some(*v);
                            }
                            _ => todo!(),
                        },
                        "position_long" => match field.value() {
                            fitparser::Value::SInt32(v) => {
                                longitude = Some(*v);
                            }
                            _ => todo!(),
                        },
                        _ => {}
                    }
                }
                if let (Some(latitude), Some(longitude)) = (latitude, longitude) {
                    let point = lat_and_long(latitude, longitude);
                    let point = Waypoint::new(point);
                    segment.points.push(point);
                } else if let (None, None) = (latitude, longitude) {
                    continue;
                } else {
                    println!("{:?}, {:?}", latitude, longitude);
                    panic!();
                }
            }
            _ => {}
        };
    }

    track.segments.push(segment);
    let mut data: Gpx = Gpx {
        version: GpxVersion::Gpx11,
        ..Default::default()
    };
    data.tracks.push(track);

    let file =
        File::create(Path::new("gpx").join(path_clone.file_name().unwrap().to_str().unwrap()))
            .unwrap();

    gpx::write(&data, file).unwrap();
}

fn lat_and_long(latitude: i32, longitude: i32) -> Point {
    Point::new(
        semicircles_to_degrees(longitude),
        semicircles_to_degrees(latitude),
    )
}

fn semicircles_to_degrees(semicircles: i32) -> f64 {
    (BigDecimal::from(semicircles) * BigDecimal::from_str("180").unwrap() / i32::MAX)
        .to_f64()
        .unwrap()
}

fn read_test_data() -> Vec<PathBuf> {
    fs::read_dir("/home/toby/git/rust/fit-decoder/fit_files/activity_files")
        .unwrap()
        .map(|r| r.unwrap())
        .map(|d| d.path())
        .collect()
}
