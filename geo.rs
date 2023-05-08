
pub mod calculations {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    pub fn distance(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {

        let radians1 = lat1.to_radians();
        let radians2 = lat2.to_radians();

        let delta_latitude = (lat1 - lat2).to_radians();
        let delta_longitude = (long1 - long2).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + radians1.cos()
            * radians2.cos()
            * f64::powi((delta_longitude/2.0).sin(), 2);

        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

        distance
    }
}
