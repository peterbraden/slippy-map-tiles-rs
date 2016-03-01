#[derive(PartialEq, Eq, Debug)]
pub struct Tile {
    zoom: u8,
    x: u32,
    y: u32,
}

impl Tile {
    fn new(zoom: u8, x: u32, y: u32) -> Option<Tile> {
        if x < 2^(zoom as u32) && y < 2^(zoom as u32) {
            Some(Tile { zoom: zoom, x: x, y: y })
        } else {
            None
        }
    }

    fn parent(&self) -> Option<Tile> {
        match self.zoom {
            0 => {
                // zoom 0, no parent
                None
            },
            _ => {
                Tile::new(self.zoom-1, self.x/2, self.y/2)
            }
        }
    }

    fn subtiles(&self) -> Option<[Tile; 4]> {
        match self.zoom {
            std::u8::MAX => {
                None
            },
            _ => {
                let z = self.zoom+1;
                let x = 2*self.x;
                let y = 2*self.y;
                Some([Tile{zoom:z, x:x, y:y}, Tile{zoom:z, x:x+1, y:y}, Tile{zoom:z, x:x, y:y+1}, Tile{zoom:z, x:x+1, y:y+1}])
            }
        }
    }

    fn centre_point(&self) -> LatLon {
        let n: f32 = 2f32.powi(self.zoom as i32);
        let lon_deg: f32 = (self.x as f32) / n * 360f32 - 180f32;
        let lat_rad: f32 = ((1f32 - 2f32 * (self.y as f32) / n) * std::f32::consts::PI).sinh().atan();
        let lat_deg: f32 = lat_rad * 180f32 * std::f32::consts::FRAC_1_PI;

        // FIXME figure out the unwrapping here....
        // Do we always know it's valid?
        LatLon::new(lat_deg, lon_deg).unwrap()

    }

    fn center_point(&self) -> LatLon {
        self.centre_point()
    }

    fn tc_path<T: std::fmt::Display>(&self, ext: T) -> String {
        let tc = xy_to_tc(self.x, self.y);
        format!("{}/{}/{}/{}/{}/{}/{}.{}", self.zoom, tc[0], tc[1], tc[2], tc[3], tc[4], tc[5], ext)
    }

}

#[derive(PartialEq, Debug)]
pub struct LatLon {
    lat: f32,
    lon: f32,
}

impl LatLon {
    fn new(lat: f32, lon: f32) -> Option<LatLon> {
        if lat <= 90f32 && lat >= -90f32 && lon <= 180f32 && lon >= -180f32 {
            Some(LatLon{ lat: lat, lon: lon })
        } else {
            None
        }
    }

}

pub fn xy_to_tc(x: u32, y: u32) -> [String; 6] {
    [
        format!("{:03}", x/1_000_000),
        format!("{:03}", (x / 1_000) % 1_000),
        format!("{:03}", x % 1_000),
        format!("{:03}", y/1_000_000),
        format!("{:03}", (y / 1_000) % 1_000),
        format!("{:03}", y % 1_000),
    ]
}

pub fn zxy_to_tc_path(z: u8, x: u32, y: u32, ext: &str) -> String {
    let tc = xy_to_tc(x, y);
    format!("{}/{}/{}/{}/{}/{}/{}.{}", z, tc[0], tc[1], tc[2], tc[3], tc[4], tc[5], ext)
}

pub fn xy_to_mp(x: u32, y: u32) -> [String; 4] {
    [
        format!("{:04}", x/10_000),
        format!("{:04}", x % 10_000),
        format!("{:04}", y/10_000),
        format!("{:04}", y % 10_000),
    ]
}

pub fn zxy_to_mp_path(z: u8, x: u32, y: u32, ext: &str) -> String {
    let mp = xy_to_mp(x, y);
    format!("{}/{}/{}/{}/{}.{}", z, mp[0], mp[1], mp[2], mp[3], ext)
}

// TODO do mod_tile tile format

mod test {
    use super::*;

    #[test]
    fn tc() {
        let res = xy_to_tc(3, 4);
        assert_eq!(res[0], "000");
        assert_eq!(res[1], "000");
        assert_eq!(res[2], "003");
        assert_eq!(res[3], "000");
        assert_eq!(res[4], "000");
        assert_eq!(res[5], "004");

        assert_eq!(zxy_to_tc_path(2, 3, 4, "png"), "2/000/000/003/000/000/004.png".to_string());
    }

    #[test]
    fn mp() {
        let res = xy_to_mp(3, 4);
        assert_eq!(res[0], "0000");
        assert_eq!(res[1], "0003");
        assert_eq!(res[2], "0000");
        assert_eq!(res[3], "0004");

        assert_eq!(zxy_to_mp_path(2, 3, 4, "png"), "2/0000/0003/0000/0004.png".to_string());
    }

    #[test]
    fn tiles() {
        let tile = Tile::new(1, 5, 5);
        assert_eq!(tile.is_none(), true);

        let tile = Tile::new(1, 0, 0);
        assert_eq!(tile.is_none(), false);
        let tile = tile.unwrap();
        let parent = tile.parent();
        assert_eq!(parent.is_none(), false);
        let parent = parent.unwrap();
        assert_eq!(parent, Tile::new(0, 0, 0).unwrap());

        assert_eq!(parent.centre_point(), LatLon::new(0f32, 0f32).unwrap());
        assert_eq!(parent.tc_path("png"), "0/000/000/000/000/000/000.png");

        let children = parent.subtiles();
        assert_eq!(children.is_none(), false);
        let children: [Tile; 4] = children.unwrap();
        assert_eq!(children[0], Tile::new(1, 0, 0).unwrap());
        assert_eq!(children[0].tc_path("png"), "1/000/000/000/000/000/000.png");

        assert_eq!(children[1], Tile::new(1, 1, 0).unwrap());
        assert_eq!(children[1].tc_path("png"), "1/000/000/001/000/000/000.png");

        assert_eq!(children[2], Tile::new(1, 0, 1).unwrap());
        assert_eq!(children[2].tc_path("png"), "1/000/000/000/000/000/001.png");

        assert_eq!(children[3], Tile::new(1, 1, 1).unwrap());
        assert_eq!(children[3].tc_path("png"), "1/000/000/001/000/000/001.png");
        


    }
}
