// WTF ???
use std::f64::consts::PI;

// gravity acceleration
const GRAVITY_ACC: f64 = 9.81 * 3.6 * 60.0;
// force applied by air on the cyclist
const DRAG: f64 = 60.0 * 0.3 / 3.6;
// in minutes
const DELTA_T: f64 = 1.0 / 60.0;
// pedaling thrust
const G_THRUST: f64 = 60.0 * 3.6 * 3.6;
// biker's mass
const MASS: f64 = 80.0;
// initial biker's power
const WATTS0: f64 = 225.0;
// loss of power at each deltaT
const D_WATTS: f64 = 0.5;

//fn temps(v0: i32, slope: i32, d_tot: i32) -> i32 {
//    let mut v = v0 as f64;
//    let weight_drag = (slope as f64 / 100.0).atan().sin();
//    let mut delta_accumulate = 0.0;
//    let mut current_power;
//    let mut x = 0.0;
//    loop {
//        // m / s^2 == J / MASS
//        print!(
//            "power: {:.5}, loss: {:.5}, weight_drag: {:.5}, air_drag: {:.5} ",
//            WATTS0 / MASS * 3.6 * 60.0,
//            WATTS0.min(D_WATTS * delta_accumulate * DELTA_T) / MASS * 3.6 * 60.0,
//            weight_drag,
//            DRAG * v * v / MASS
//        );
//        current_power = WATTS0 - delta_accumulate * D_WATTS / MASS * 3.6 * 60.0 - weight_drag;
//        if v > 1e-5 {
//            current_power -= DRAG * v * v / MASS;
//        }
//
//        if v > 0.0 && current_power > 0.0 {
//            current_power += G_THRUST * current_power / (v * MASS);
//        }
//
//        if current_power.abs() < 1e-5 {
//            current_power = 0.0;
//        }
//
//        x += v / 3600.0 ;
//        print!("cur_power: {:.5}, v: {:.5} ", current_power, v);
//        if x > d_tot as f64 {
//            return (delta_accumulate / 60.0 ).ceil() as i32;
//        }
//        if v - 3.0 <= 1e-2 {
//            return -1;
//        }
//        println!("d_tot, d_al = {}, {:.5}", d_tot, x);
//        delta_accumulate += 1.0;
//        v += current_power / 60.0 ;
//    }
//}

fn temps(v0: i32, slope: i32, d_tot: i32) -> i32 {
    let slope = (slope as f64 / 100.0).atan().sin();
    let d_tot = d_tot as f64;
    let mut t = 0.0;
    let mut v = v0 as f64;
    let mut d = 0.0;

    while d < d_tot {
        let w = WATTS0 - t * D_WATTS;
        let a = G_THRUST * w / (v * MASS) - GRAVITY_ACC * slope - DRAG * v * v / MASS;

        t += DELTA_T;
        v += a * DELTA_T;
        d += v * DELTA_T / 60.0;

        if v - 3.0 <= 1e-2 {
            return -1;
        }
    }

    t.round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(v0: i32, slope: i32, d_tot: i32, exp: i32) -> () {
        assert_eq!(exp, temps(v0, slope, d_tot))
    }

    #[test]
    fn basic_tests() {
        dotest(30, 5, 30, 114);
        dotest(30, 20, 30, -1);
        dotest(30, 8, 20, 110);
        dotest(30, 0, 5, 9);
        dotest(50, 10, 25, 185);
    }
}
