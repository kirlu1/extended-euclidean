use std::collections::BTreeMap;

fn main() {
    let q2_2_1 = repr_gcd(101, 17);
    // Modular inverse of 17 mod 101
    println!("{}", q2_2_1);

    let q2_2_2 = repr_gcd(357, 1234);
    // Modular inverse of 357 mod 1234
    println!("{}", q2_2_2);

    let q2_2_3 = repr_gcd(3125, 9987);
    // Modular inverse of 3125 mod 9987
    println!("{}", q2_2_3);

    
    let q2_4_1 = repr_gcd(13, 99);
    // Modular inverse of 13 mod 99
    println!("{}", q2_4_1);

    let q2_4_2 = repr_gcd(15, 101);
    // Modular inverse of 15 mod 101
    println!("{}", q2_4_2);

    let mut q2_4_3 = repr_gcd(101, 99);
    // Modular inverse of 99 mod 101
    println!("{}", q2_4_3);

    while q2_4_3.times_b < 0 {
        q2_4_3.times_a -= q2_4_3.b;
        q2_4_3.times_b += q2_4_3.a;
    }
    // Modular inverse of 101 mod 99
    println!("{}", q2_4_3);
}


// represent the gcd of a and b as a * x + b * y
fn repr_gcd(a : i32, b : i32) -> DiophanticRepr {
    let subreprs = gcd_chain(a,b);

    let mut r = subreprs
        .first_key_value()
        .expect("empty map").1.clone();

    while (a.max(b), a.min(b)) != (r.a.max(r.b), r.a.min(r.b)){
        let subbed = r.a.min(r.b);
        let other = subreprs.get(&subbed)
            .expect(&format!("value should exist for {}", subbed));

        r = r.substitute(other.clone());
    }

    if r.b < r.a {
        while r.times_b < 0 {
            r.times_b += r.a;
            r.times_a -= r.b;
        }
    } else {
        while r.times_a < 0 {
            r.times_a += r.b;
            r.times_b -= r.a;
        }
    }

    r
} 


#[derive(Debug, Clone)]
struct DiophanticRepr {
    a : i32,
    times_a : i32,
    b : i32,
    times_b : i32,
}

impl std::fmt::Display for DiophanticRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} = {} * {} + {} * {}", 
            self.val(), 
            self.a, 
            self.times_a,
            self.b,
            self.times_b
        )
    }
}

impl DiophanticRepr {
    fn new(a : i32, times_a : i32, b : i32, times_b : i32) -> Self {
        DiophanticRepr {
            a, 
            times_a,
            b,
            times_b,
        }
    }

    fn val(&self) -> i32 {
        self.a * self.times_a + self.b * self.times_b
    }

    fn substitute(self, other : DiophanticRepr) -> DiophanticRepr {
        let new_a : i32;
        let new_times_a : i32;
        let new_b : i32;
        let new_times_b : i32;

        match (self, other) {
            (lhs, rhs) if lhs.a == rhs.val() && lhs.b == rhs.a => {
                new_a = rhs.b;
                new_times_a = lhs.times_a * rhs.times_b;
                new_b = lhs.b;
                new_times_b = lhs.times_a * rhs.times_a + lhs.times_b;

            },
            (lhs, rhs) if lhs.a == rhs.val() && lhs.b == rhs.b => {
                new_a = rhs.a;
                new_times_a = lhs.times_a * rhs.times_a;
                new_b = lhs.b;
                new_times_b = lhs.times_a * rhs.times_b + lhs.times_b;
            },
            (lhs, rhs) if lhs.b == rhs.val() && lhs.a == rhs.a => {
                new_b = rhs.b;
                new_times_b = lhs.times_b * rhs.times_b;
                new_a = lhs.a;
                new_times_a = lhs.times_b * rhs.times_a + lhs.times_a;
            },
            (lhs, rhs) if lhs.b == rhs.val() && lhs.a == rhs.b => {
                new_b = rhs.a;
                new_times_b = lhs.times_b * rhs.times_a;
                new_a = lhs.a;
                new_times_a = lhs.times_b * rhs.times_b + lhs.times_a;
            },
            (_,_) => panic!(),
        }

        DiophanticRepr::new(new_a, new_times_a, new_b, new_times_b)
    }
}

fn gcd_chain(a : i32, b : i32) -> BTreeMap<i32, DiophanticRepr> {
    gcd_aux(a, b, BTreeMap::new())
}

fn gcd_aux(
    a : i32, 
    b : i32, 
    mut btmap : BTreeMap<i32, DiophanticRepr>
) -> BTreeMap<i32, DiophanticRepr> 
{
    let big = a.max(b);
    let small = a.min(b);

    let divided = big / small;
    let rem = big - divided * small;
    
    if rem == 0 { return btmap };

    let repr = DiophanticRepr::new(big, 1, small, -divided);

    btmap.insert(rem, repr);

    gcd_aux(small, rem, btmap)
}