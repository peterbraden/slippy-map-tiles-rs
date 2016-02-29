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
        format!("{:03}", x/10_000),
        format!("{:03}", x % 10_000),
        format!("{:03}", y/10_000),
        format!("{:03}", y % 10_000),
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

    fn mp() {
        let res = xy_to_mp(3, 4);
        assert_eq!(res[0], "0000");
        assert_eq!(res[2], "0003");
        assert_eq!(res[4], "0000");
        assert_eq!(res[5], "0004");

        assert_eq!(zxy_to_mp_path(2, 3, 4, "png"), "2/0000/0003/0000/0004.png".to_string());
    }
}
