#[derive(PartialEq, Eq, Debug)]
pub struct Tile {
    zoom: u8,
    x: u32,
    y: u32,
}

impl Tile {
    pub fn new(zoom: u8, x: u32, y: u32) -> Option<Tile> {
        if x < 2u32.pow(zoom as u32) && y < 2u32.pow(zoom as u32) {
            Some(Tile { zoom: zoom, x: x, y: y })
        } else {
            None
        }
    }

    pub fn parent(&self) -> Option<Tile> {
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

    pub fn subtiles(&self) -> Option<[Tile; 4]> {
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

    pub fn centre_point(&self) -> LatLon {
        tile_nw_lat_lon(self.zoom, (self.x as f32)+0.5, (self.y as f32)+0.5)
    }

    pub fn center_point(&self) -> LatLon {
        self.centre_point()
    }

    pub fn nw_corner(&self) -> LatLon {
        tile_nw_lat_lon(self.zoom, (self.x as f32), (self.y as f32))
    }

    pub fn ne_corner(&self) -> LatLon {
        tile_nw_lat_lon(self.zoom, (self.x as f32)+1.0, (self.y as f32))
    }

    pub fn sw_corner(&self) -> LatLon {
        tile_nw_lat_lon(self.zoom, (self.x as f32), (self.y as f32)+1.0)
    }

    pub fn se_corner(&self) -> LatLon {
        tile_nw_lat_lon(self.zoom, (self.x as f32)+1.0, (self.y as f32)+1.0)
    }

    pub fn top(&self) -> f32 {
        self.nw_corner().lat
    }
    pub fn bottom(&self) -> f32 {
        self.sw_corner().lat
    }
    pub fn left(&self) -> f32 {
        self.nw_corner().lon
    }
    pub fn right(&self) -> f32 {
        self.se_corner().lon
    }

    pub fn tc_path<T: std::fmt::Display>(&self, ext: T) -> String {
        let tc = xy_to_tc(self.x, self.y);
        format!("{}/{}/{}/{}/{}/{}/{}.{}", self.zoom, tc[0], tc[1], tc[2], tc[3], tc[4], tc[5], ext)
    }

    pub fn mp_path<T: std::fmt::Display>(&self, ext: T) -> String {
        let mp = xy_to_mp(self.x, self.y);
        format!("{}/{}/{}/{}/{}.{}", self.zoom, mp[0], mp[1], mp[2], mp[3], ext)
    }

    pub fn all() -> AllTilesIterator {
        AllTilesIterator{ next_zoom: 0, next_x: 0, next_y: 0}
    }

    pub fn bbox(&self) -> BBox {
        let nw = self.nw_corner();
        let se = self.se_corner();

        BBox::new_from_points(&nw, &se)
    }

}

pub struct AllTilesIterator {
    next_zoom: u8,
    next_x: u32,
    next_y: u32,
}

impl Iterator for AllTilesIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        let tile = Tile::new(self.next_zoom, self.next_x, self.next_y);
        let max_tile_no = 2u32.pow(self.next_zoom as u32) - 1;
        if self.next_y < max_tile_no {
            self.next_y += 1;
        } else if self.next_x < max_tile_no {
            self.next_x += 1;
            self.next_y = 0;
        } else  if self.next_zoom < std::u8::MAX {
            self.next_zoom += 1;
            self.next_x = 0;
            self.next_y = 0;
        }

        tile
    }
}

fn tile_nw_lat_lon(zoom: u8, x: f32, y: f32) -> LatLon {
    let n: f32 = 2f32.powi(zoom as i32);
    let lon_deg: f32 = (x as f32) / n * 360f32 - 180f32;
    let lat_rad: f32 = ((1f32 - 2f32 * (y as f32) / n) * std::f32::consts::PI).sinh().atan();
    let lat_deg: f32 = lat_rad * 180f32 * std::f32::consts::FRAC_1_PI;

    // FIXME figure out the unwrapping here....
    // Do we always know it's valid?
    LatLon::new(lat_deg, lon_deg).unwrap()
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

#[derive(PartialEq, Debug)]
pub struct BBox {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

impl BBox {
    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Option<BBox> {
        //let top = if top > bottom { top } else { bottom };
        //let bottom = if top > bottom { bottom } else { top };
        //let left = if right > left { left } else { right };
        //let right = if right > left { right } else { left };

        if top <= 90. && top >= -90. && bottom <= 90. && bottom >= -90.
             && left <= 180. && left >= -180. && right <= 180. && right >= -180. {
             Some(BBox{ top: top, left: left, bottom: bottom, right: right })
        } else {
            None
        }
    }

    pub fn new_from_points(topleft: &LatLon, bottomright: &LatLon) -> BBox {
        BBox{ top: topleft.lat, left: topleft.lon, bottom: bottomright.lat, right: bottomright.lon }
    }

    pub fn contains_point(&self, point: &LatLon) -> bool {
        (point.lat <= self.top && point.lat >= self.bottom && point.lon >= self.left && point.lon <= self.right)
    }

}


fn xy_to_tc(x: u32, y: u32) -> [String; 6] {
    [
        format!("{:03}", x/1_000_000),
        format!("{:03}", (x / 1_000) % 1_000),
        format!("{:03}", x % 1_000),
        format!("{:03}", y/1_000_000),
        format!("{:03}", (y / 1_000) % 1_000),
        format!("{:03}", y % 1_000),
    ]
}

fn xy_to_mp(x: u32, y: u32) -> [String; 4] {
    [
        format!("{:04}", x/10_000),
        format!("{:04}", x % 10_000),
        format!("{:04}", y/10_000),
        format!("{:04}", y % 10_000),
    ]
}

// TODO do mod_tile tile format

mod test {

    #[test]
    fn tc() {
        use super::xy_to_tc;

        let res = xy_to_tc(3, 4);
        assert_eq!(res[0], "000");
        assert_eq!(res[1], "000");
        assert_eq!(res[2], "003");
        assert_eq!(res[3], "000");
        assert_eq!(res[4], "000");
        assert_eq!(res[5], "004");
    }

    #[test]
    fn mp() {
        use super::xy_to_mp;

        let res = xy_to_mp(3, 4);
        assert_eq!(res[0], "0000");
        assert_eq!(res[1], "0003");
        assert_eq!(res[2], "0000");
        assert_eq!(res[3], "0004");
    }

    #[test]
    fn tiles_parsing() {
        use super::Tile;

        let tile = Tile::new(1, 5, 5);
        assert_eq!(tile.is_none(), true);

        assert_eq!(Tile::new(4, 8, 9).is_some(), true);

        let tile = Tile::new(1, 0, 0);
        assert_eq!(tile.is_none(), false);
    }

    #[test]
    fn tiles() {
        use super::{Tile, LatLon};
        let tile = Tile::new(1, 0, 0);

        let tile = tile.unwrap();
        let parent = tile.parent();
        assert_eq!(parent.is_none(), false);
        let parent = parent.unwrap();
        assert_eq!(parent, Tile::new(0, 0, 0).unwrap());

        assert_eq!(parent.centre_point(), LatLon::new(0f32, 0f32).unwrap());
        assert_eq!(parent.nw_corner(), LatLon::new(85.05112, -180.0).unwrap());
        assert_eq!(parent.ne_corner(), LatLon::new(85.05112, 180.0).unwrap());
        assert_eq!(parent.sw_corner(), LatLon::new(-85.05112, -180.0).unwrap());
        assert_eq!(parent.se_corner(), LatLon::new(-85.05112, 180.0).unwrap());

        assert_eq!(parent.top(), 85.05112);
        assert_eq!(parent.bottom(), -85.05112);
        assert_eq!(parent.left(), -180.0);
        assert_eq!(parent.right(), 180.0);


        assert_eq!(parent.tc_path("png"), "0/000/000/000/000/000/000.png");
        assert_eq!(parent.mp_path("png"), "0/0000/0000/0000/0000.png");

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

    #[test]
    fn all_tiles() {
        use super::Tile;

        let mut it = Tile::all();

        assert_eq!(it.next(), Tile::new(0, 0, 0));
        assert_eq!(it.next(), Tile::new(1, 0, 0));
        assert_eq!(it.next(), Tile::new(1, 0, 1));
        assert_eq!(it.next(), Tile::new(1, 1, 0));
        assert_eq!(it.next(), Tile::new(1, 1, 1));
        assert_eq!(it.next(), Tile::new(2, 0, 0));
        assert_eq!(it.next(), Tile::new(2, 0, 1));
        assert_eq!(it.next(), Tile::new(2, 0, 2));
        assert_eq!(it.next(), Tile::new(2, 0, 3));
        assert_eq!(it.next(), Tile::new(2, 1, 0));

        let it = Tile::all();
        let z5_tiles: Vec<Tile> = it.skip_while(|t| { t.zoom < 5 }).take(1).collect();
        assert_eq!(z5_tiles[0], Tile::new(5, 0, 0).unwrap());

    }

    #[test]
    fn bbox_create() {
        use super::{BBox, LatLon};

        // left=5.53 bottom=47.23 right=15.38 top=54.96
        let b1: Option<BBox> = BBox::new(54.9, 5.5, 47.2, 15.38);
        assert!(b1.is_some());
        let b1 = b1.unwrap();
        assert_eq!(b1.top, 54.9);

        let p1 = LatLon::new(54.9, 5.5).unwrap();
        let p2 = LatLon::new(47.2, 15.38).unwrap();
        let b2: BBox = BBox::new_from_points(&p1, &p2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn bbox_tile() {
        use super::{BBox, Tile};
        let t = Tile::new(0, 0, 0).unwrap();
        assert_eq!(t.bbox(), BBox::new(85.05112, -180., -85.05112, 180.).unwrap());
    }

    #[test]
    fn bbox_contains_point(){
        use super::{Tile, LatLon};
        // triangle from London, to Bristol to Birmingham
        let tile = Tile::new(7, 63, 42).unwrap();
        let bbox = tile.bbox();
        let point1 = LatLon::new(51.75193, -1.25781).unwrap();
        let point2 = LatLon::new(48.7997, 2.4218).unwrap();
        
        assert!(bbox.contains_point(&point1));
        assert!(!bbox.contains_point(&point2));

    }
}
