pub(crate) fn cube_root(x: f64, y: f64) -> bool { //funkcja jest niemalejąca więc możemy użyć funkcji przeciwnej do sprawdzenia czy punkt leży pod wykresem (będzie szybciej)
    return if y.powf(3.0) < x {
        true
    } else {
        false
    };
}

pub(crate) fn sinus(x: f64, y: f64) -> bool {
    return if x.sin() > y {
        true
    } else {
        false
    };
}

pub(crate) fn wielomian(x: f64, y: f64) -> bool {
    return if 4.0*x*(1.0-x).powf(3.0) > y {
        true
    } else {
        false
    };
}

pub(crate) fn circle(x: f64, y: f64) -> bool {
    return if x.powf(2.0) + (y-1.0).powf(2.0) < 1.0 {
        true
    } else {
        false
    };
}