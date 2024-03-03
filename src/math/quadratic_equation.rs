pub enum Solutions {
    None,
    One(f32),
    Two(f32, f32)
}

pub fn solve(a: f32, b: f32, c: f32) -> Solutions {
    let delta = b*b - 4.0*a*c;

    if delta < 0.0 {
        return Solutions::None;
    }

    if delta == 0.0 {
        return Solutions::One(-b/2.0*a);
    }

    let delta_sqrt = delta.sqrt();
    let two_a = 2.0*a;

    return Solutions::Two(
        (-b - delta_sqrt)/two_a,
        (-b + delta_sqrt)/two_a
    );
}
