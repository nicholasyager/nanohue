use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ColorCoordinate {
    x: f32,
    y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HSVColor {
    hue: u32,
    saturation: u8,
    value: u8,
}

fn gamma_correction(x: f32) -> f32 {
    if x <= 0.0031308 {
        return 12.92 * x;
    }

    (1.0 + 0.055) * f32::powf(x, 1.0 / 2.4) - 0.055
}

#[derive(Debug)]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGBColor {
    pub fn from_coordinate(color: ColorCoordinate, gamut: ColorGamut, brightness: u8) -> RGBColor {
        let mut xy_point = color;

        if !check_point_in_lamps_reach(xy_point, gamut) {
            // Calculate the closest point on the color gamut triangle
            // and use that as xy value See step 6 of color to xy.
            xy_point = get_closest_point_to_point(gamut, xy_point);
        }

        // Calculate XYZ values Convert using the following formulas:
        let y = brightness as f32 / 255_f32;
        let x = (y / xy_point.y) * xy_point.x;
        let z = (y / xy_point.y) * (1_f32 - xy_point.x - xy_point.y);

        // Convert to RGB using Wide RGB D65 conversion
        let mut r = x * 1.656492 - y * 0.354851 - z * 0.255038;
        let mut g = -x * 0.707196 + y * 1.655397 + z * 0.036152;
        let mut b = x * 0.051713 - y * 0.121364 + z * 1.011530;

        // Apply reverse gamma correction
        r = gamma_correction(r);
        g = gamma_correction(g);
        b = gamma_correction(b);

        // Bring all negative components to zero
        r = f32::max(r, 0_f32);
        g = f32::max(g, 0_f32);
        b = f32::max(b, 0_f32);

        let values = [r, g, b];

        // Option<f32> is returned because the slice might be empty
        let max_value: Option<f32> = values.iter().copied().fold(None, |max, current| match max {
            Some(max) => Some(max.max(current)),
            None => Some(current),
        });

        let max_component = match max_value {
            Some(max) => max,
            None => panic!("No max?!?"),
        };

        // If one component is greater than 1, weight components by that value.
        if max_component > 1.0 {
            r = r / max_component;
            g = g / max_component;
            b = b / max_component;
        }

        RGBColor {
            red: (r * 255.0).clamp(0.0, 255.0) as u8,
            green: (g * 255.0).clamp(0.0, 255.0) as u8,
            blue: (b * 255.0).clamp(0.0, 255.0) as u8,
        }
    }
}

pub type ColorGamut = [[f32; 2]; 3];

// struct Palette {}

fn cross_product(point1: ColorCoordinate, point2: ColorCoordinate) -> f32 {
    // Returns the cross product of two XYPoints.
    return point1.x * point2.y - point1.y * point2.x;
}

fn check_point_in_lamps_reach(color: ColorCoordinate, gamut: ColorGamut) -> bool {
    // Check if the provided XYPoint can be recreated by a Hue lamp.

    let v1 = ColorCoordinate {
        x: gamut[1][0] - gamut[0][0],
        y: gamut[1][1] - gamut[0][1],
    };
    let v2 = ColorCoordinate {
        x: gamut[2][0] - gamut[0][0],
        y: gamut[2][1] - gamut[0][1],
    };

    let q = ColorCoordinate {
        x: color.x - gamut[0][0],
        y: color.y - gamut[0][1],
    };
    let s = cross_product(q, v2) / cross_product(v1, v2);
    let t = cross_product(v1, q) / cross_product(v1, v2);

    return (s >= 0.0) && (t >= 0.0) && (s + t <= 1.0);
}

fn get_closest_point_to_line(
    a: ColorCoordinate,
    b: ColorCoordinate,
    p: ColorCoordinate,
) -> ColorCoordinate {
    // Find the closest point on a line. This point will be reproducible by a Hue lamp.
    let ap = ColorCoordinate {
        x: p.x - a.x,
        y: p.y - a.y,
    };
    let ab = ColorCoordinate {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let mut t: f32 = ap_ab / ab2;

    if t < 0.0 {
        t = 0.0
    } else if t > 1.0 {
        t = 1.0
    }

    ColorCoordinate {
        x: a.x + ab.x * t,
        y: a.y + ab.y * t,
    }
}

fn get_distance_between_two_points(one: ColorCoordinate, two: ColorCoordinate) -> f32 {
    // Returns the distance between two XYPoints.
    let dx = one.x - two.x;
    let dy = one.y - two.y;
    f32::sqrt(dx * dx + dy * dy)
}

fn get_closest_point_to_point(gamut: ColorGamut, xy_point: ColorCoordinate) -> ColorCoordinate {
    // Color is unreproducible, find the closest point on each line in the CIE 1931 'triangle'.
    let p_ab = get_closest_point_to_line(
        ColorCoordinate {
            x: gamut[0][0],
            y: gamut[0][1],
        },
        ColorCoordinate {
            x: gamut[1][0],
            y: gamut[1][1],
        },
        xy_point,
    );
    let p_ac = get_closest_point_to_line(
        ColorCoordinate {
            x: gamut[2][0],
            y: gamut[2][1],
        },
        ColorCoordinate {
            x: gamut[0][0],
            y: gamut[0][1],
        },
        xy_point,
    );
    let p_bc = get_closest_point_to_line(
        ColorCoordinate {
            x: gamut[1][0],
            y: gamut[1][1],
        },
        ColorCoordinate {
            x: gamut[2][0],
            y: gamut[2][1],
        },
        xy_point,
    );

    // Get the distances per point and see which point is closer to our Point.
    let d_ab = get_distance_between_two_points(xy_point, p_ab);
    let d_ac = get_distance_between_two_points(xy_point, p_ac);
    let d_bc = get_distance_between_two_points(xy_point, p_bc);

    let mut lowest = d_ab;
    let mut closest_point = p_ab;

    if d_ac < lowest {
        lowest = d_ac;
        closest_point = p_ac;
    }

    if d_bc < lowest {
        // lowest = d_bc;
        closest_point = p_bc;
    }

    // Change the xy value to a value which is within the reach of the lamp.
    let cx = closest_point.x;
    let cy = closest_point.y;

    ColorCoordinate { x: cx, y: cy }
}

// r = X * 1.656492 - Y * 0.354851 - Z * 0.255038
// g = -X * 0.707196 + Y * 1.655397 + Z * 0.036152
// b = X * 0.051713 - Y * 0.121364 + Z * 1.011530
