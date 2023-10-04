use std::fs;


fn main() {
    let contents = fs::read_to_string("map.osm").unwrap();
    // let string = "schnitzel\"schnitzel";
    // let index = string.chars().position(|x| x == '"').unwrap();
    //println!("{:?}", find_addr(contents));
    println!("{:?}", find_addr_vec(contents));
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
                    println!("{}", contents_lines[line_index + 2]);
                    panic!();
                };
                //let street_name_index = contents_lines[line_index + 2].find("v=").unwrap() + 3;
                addr_vec.push((lat, lon, street_name));
        }
    }
    return addr_vec;
}

fn find_street_name(lat: f64, lon: f64, addr_vec: Vec<(f64, f64, String)>) -> String {
    //use haversine distance
    return String::from("");
}
