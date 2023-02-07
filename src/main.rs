fn main() {
    let y = 10;
    add_one(y);
}

fn add_one(x: i32) -> i32 {
    println!("Added one!");
    x + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_add_one() {
        let t = 10;
        assert_eq!(add_one(t), 11);
    }
}
