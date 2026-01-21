//fn sqrt(x: f32) -> f32 {
//    x * x
//}
//
//fn is_inside_circle(center: (usize, usize), tile: (usize, usize), radius: f32) -> bool {
//    let dx = center.0 - tile.0;
//    let dy = center.1 - tile.1;
//    let distance = dx * dx + dy * dy;
//    distance as f32 <= (radius)
//}

fn draw_loading(center: (usize, usize), radius: f32, rael: &mut Rael, angle: f32) {
    //let top = (center.1 as f32 - radius).ceil() as u16;
    //let bottom = (center.1 as f32 + radius).floor() as u16;
    //for y in top..bottom {
    //    let dy = y as i32 - center.1 as i32;
    //    let dx = (radius * radius - (dy * dy) as f32).sqrt();
    //
    //    let left = (center.0 as f32 - dx).ceil() as u16;
    //    let right = (center.0 as f32 + dx).floor() as u16;
    //    for x in left..right {
    //        rael.screen
    //            .set_pixel(x.into(), y.into(), 0, Color::new(255, 255, 255));
    //    }
    //}
    for r in 0..=(radius * f32::sqrt(0.5)).floor() as usize {
        let d = (radius * radius - (r * r) as f32).sqrt().floor() as usize;
        let color = Color::new(255, 255, 255);

        rael.screen
            .set_pixel(center.0.saturating_sub(d), center.1 + r, 1, color);
        rael.screen.set_pixel(center.0 + d, center.1 + r, 1, color);
        rael.screen.set_pixel(
            center.0.saturating_sub(d),
            center.1.saturating_sub(r),
            1,
            color,
        );
        rael.screen
            .set_pixel(center.0 + d, center.1.saturating_sub(r), 1, color);
        rael.screen
            .set_pixel(center.0 + r, center.1.saturating_sub(d), 1, color);
        rael.screen.set_pixel(center.0 + r, center.1 + d, 1, color);
        rael.screen.set_pixel(
            center.0.saturating_sub(r),
            center.1.saturating_sub(d),
            1,
            color,
        );
        rael.screen
            .set_pixel(center.0.saturating_sub(r), center.1 + d, 1, color);
    }
    draw_cone(center, angle, f32::consts::FRAC_PI_4, radius - 2.0, rael);
}

pub fn draw_cone(
    center: (usize, usize),
    angle: f32, // where the cone points (radians)
    width: f32, // half-angle (radians)
    radius: f32,
    rael: &mut Rael,
) {
    let cx = center.0 as f32;
    let cy = center.1 as f32;
    let r = radius.ceil() as i32;

    for oy in -r..=r {
        for ox in -r..=r {
            let px = cx + ox as f32;
            let py = cy + oy as f32;

            if px < 0.0 || py < 0.0 {
                continue;
            }

            let dx = px - cx;
            let dy = py - cy;

            let dist = (dx * dx + dy * dy).sqrt();
            if dist > radius {
                continue;
            }

            let pixel_angle = dy.atan2(dx);

            // shortest angular distance
            let diff = (pixel_angle - angle + PI).rem_euclid(2.0 * PI) - PI;

            if diff.abs() <= width {
                rael.screen
                    .set_pixel(px as usize, py as usize, 1, Color::new(255, 255, 255));
            }
        }
    }
}
