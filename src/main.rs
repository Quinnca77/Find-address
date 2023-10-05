use std::{fs, cmp::Ordering};

const R_EARTH: f64 = 6371000.0;

#[derive(PartialEq,PartialOrd)]
struct NonNan(f64);

impl NonNan {
    fn new(val: f64) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn main() {
    let contents = fs::read_to_string("map.osm").unwrap();
    let addr_vec = find_addr_vec(contents);
    println!("{}", find_street_name(50.816486, 6.005317, addr_vec));
}

fn find_addr_vec(contents: String) -> Vec<(f64, f64, String)> {
    let contents_lines = contents.lines().collect::<Vec<&str>>();
    let mut addr_vec: Vec<(f64, f64, String)> = Vec::new();
    for (line_index, line) in contents_lines.iter().enumerate() {
        if line.contains("addr:city") {
                let lat_index = contents_lines[line_index - 1].find("lat=").unwrap_or(200) + 4;
                let lon_index = contents_lines[line_index - 1].find("lon=").unwrap_or(200) + 4;
                if lat_index > 200 {
                    continue;
                }
                let lat_index_end = contents_lines[line_index - 1][lat_index..contents_lines[line_index - 1].len()].chars().skip(1).position(|x| x == '"').unwrap() + 1 + lat_index;
                let lon_index_end = contents_lines[line_index - 1][lon_index..contents_lines[line_index - 1].len()].chars().skip(1).position(|x| x == '"').unwrap() + 1 + lon_index;
                let lat = contents_lines[line_index - 1][(lat_index + 1)..lat_index_end].parse::<f64>().unwrap();
                let lon = contents_lines[line_index - 1][(lon_index + 1)..lon_index_end].parse::<f64>().unwrap();
                let mut street_name = String::new();
                if contents_lines[line_index + 3].contains("addr:street") {
                    let street_name_index = contents_lines[line_index + 3].find("v=").unwrap() + 3;
                    street_name = contents_lines[line_index + 3][street_name_index..(contents_lines[line_index + 3].len() - 3)].to_string();
                } else if contents_lines[line_index + 2].contains("addr:street") {
                    let street_name_index = contents_lines[line_index + 2].find("v=").unwrap() + 3;
                    street_name = contents_lines[line_index + 2][street_name_index..(contents_lines[line_index + 2].len() - 3)].to_string();
                } else if contents_lines[line_index + 4].contains("addr:street") {
                    let street_name_index = contents_lines[line_index + 4].find("v=").unwrap() + 3;
                    street_name = contents_lines[line_index + 4][street_name_index..(contents_lines[line_index + 4].len() - 3)].to_string();
                } else {
                    panic!("street_name could not be found in the usual lines");
                };
                addr_vec.push((lat, lon, street_name));
        }
    }
    return addr_vec;
}

fn find_street_name(lat: f64, lon: f64, addr_vec: Vec<(f64, f64, String)>) -> String {
    let mut distance_vec = Vec::new();
    let phi1 = lat.to_radians();
    for (lat_addr, lon_addr, street_name) in addr_vec {
        let phi2 = lat_addr.to_radians();
        let delta_phi = (lat_addr-lat).to_radians();
        let delta_lambda = (lon_addr-lon).to_radians();

        //formula haversine distance: https://www.movable-type.co.uk/scripts/latlong.html or https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/trigonometry.html
        let a = (delta_phi / 2.0).sin().powi(2) + (phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2));
        let c = 2.0 * a.sqrt().asin();
        let distance = R_EARTH * c;
        distance_vec.push((distance, street_name));
    }
    let non_nan_vec: Vec<(NonNan, String)> = distance_vec.iter().map(|x| (NonNan::new(x.0).unwrap(), x.1.clone())).collect();
    return non_nan_vec.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().1.clone();
}
