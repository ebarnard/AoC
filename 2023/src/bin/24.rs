use std::ops::RangeInclusive;

use aoc2023::nom::{int128, ws};
use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::separated_list0,
    sequence::{separated_pair, tuple},
};
use num_bigint::BigInt;
use num_integer::Integer;

const TEST: &str = include_str!("../../inputs/24-test.txt");
const REAL: &str = include_str!("../../inputs/24-real.txt");

type Point = [i128; 3];

fn main() {
    assert_eq!(part1(TEST, 7.0..=27.0), 2);
    assert_eq!(part1(REAL, 2e14..=4e14), 17776);
    assert_eq!(part2(TEST), 47);
    assert_eq!(part2(REAL), 948978092202212);
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    let point = || {
        map(
            tuple((ws(int128), tag(","), ws(int128), tag(","), ws(int128))),
            |(x, _, y, _, z)| [x, y, z],
        )
    };
    let stones = separated_list0(tag("\n"), separated_pair(point(), tag("@"), point()));

    let (s, parsed) = complete(stones)(input).unwrap();
    assert_eq!(s.len(), 0);
    parsed
}

fn part1(input: &str, boundary: RangeInclusive<f64>) -> u32 {
    let stones = parse(input);

    let mut crossings = 0;
    for (i, &([xa0, ya0, _], [dxa, dya, _])) in stones.iter().enumerate() {
        for &([xb0, yb0, _], [dxb, dyb, _]) in stones[i + 1..].iter() {
            let d = dxa * dyb - dxb * dya;
            if d == 0 {
                // Parallel lines do not cross.
                continue;
            }

            let n = dxa * dyb * xb0 - dxb * dya * xa0 + dxa * dxb * ya0 - dxa * dxb * yb0;

            let x = n as f64 / d as f64;
            let y = dya as f64 / dxa as f64 * (x - xa0 as f64) + ya0 as f64;

            let ta = (x - xa0 as f64) / dxa as f64;
            let tb = (x - xb0 as f64) / dxb as f64;

            if ta > 0.0 && tb > 0.0 && boundary.contains(&x) && boundary.contains(&y) {
                crossings += 1;
            }
        }
    }

    crossings
}

fn part2(input: &str) -> i128 {
    let stones = parse(input);

    // Solution is uniquely determined by four lines.
    let (a0, da) = stones[0];
    let (b0, db) = stones[1];
    // Ignore line 2 as it is parallel to line 1 in test data.
    let (c0, dc) = stones[3];
    let (d0, dd) = stones[4];

    // Lines must not be coplanar.
    assert_ne!(stp(da, db, dc), 0);
    assert_ne!(stp(da, db, dd), 0);

    let coeffs = |dk, k0| {
        [
            stp(da, db, dk),                   // ta * tb
            stp(sub(k0, b0), da, dk),          // ta
            stp(sub(a0, k0), db, dk),          // tb
            stp(sub(a0, k0), sub(b0, a0), dk), // constant
        ]
    };

    let [e, f, g, h] = coeffs(dc, c0).map(BigInt::from);
    let [p, q, r, s] = coeffs(dd, d0).map(BigInt::from);

    // Quadratic coefficients for `ta`.
    let ta_2 = -&f * &p + &e * &q;
    let ta_1 = -&h * &p + &g * &q - &f * &r + &e * &s;
    let ta_0 = -&h * &r + &g * &s;

    let ta_disc: BigInt = ta_1.pow(2) - 4 * &ta_2 * &ta_0;

    let ta_sqrtdisc = ta_disc.sqrt();
    // Solutions must be at integer times.
    assert_eq!(ta_sqrtdisc.pow(2), ta_disc);

    let (ta_a, ta_a_rem) = (-&ta_1 + &ta_sqrtdisc).div_rem(&(2 * &ta_2));
    let (ta_b, ta_b_rem) = (-&ta_1 - &ta_sqrtdisc).div_rem(&(2 * &ta_2));

    let tb_and_rem = |ta: &BigInt| (-ta * &f - &h).div_rem(&(ta * &e + &g));
    let (tb_a, tb_a_rem) = tb_and_rem(&ta_a);
    let (tb_b, tb_b_rem) = tb_and_rem(&ta_b);

    let (ta, tb) = if ta_a >= 0.into() && tb_a >= 0.into() && ta_a_rem == 0.into() {
        assert_eq!(tb_a_rem, 0.into());
        (ta_a, tb_a)
    } else if ta_b >= 0.into() && tb_b >= 0.into() && ta_b_rem == 0.into() {
        assert_eq!(tb_b_rem, 0.into());
        (ta_b, tb_b)
    } else {
        panic!();
    };

    let start_coord = |i| (&ta * (&tb * db[i] + b0[i]) - &tb * (&ta * da[i] + a0[i])) / (&ta - &tb);

    let x0 = start_coord(0);
    let y0 = start_coord(1);
    let z0 = start_coord(2);

    (x0 + y0 + z0).try_into().unwrap()
}

fn sub(u: Point, v: Point) -> Point {
    [u[0] - v[0], u[1] - v[1], u[2] - v[2]]
}

fn dot(u: Point, v: Point) -> i128 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn cross(u: Point, v: Point) -> Point {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

// Scalar triple product `U.(V x W)`.
fn stp(u: Point, v: Point, w: Point) -> i128 {
    dot(u, cross(v, w))
}
