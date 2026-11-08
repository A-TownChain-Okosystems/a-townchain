// Game rules and validation
pub fn is_valid_move(from: (i32, i32), to: (i32, i32), max_distance: i32) -> bool {
    let dx = (to.0 - from.0).abs();
    let dy = (to.1 - from.1).abs();
    dx + dy <= max_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move() {
        assert!(is_valid_move((0,0), (3,3), 6));
        assert!(!is_valid_move((0,0), (5,5), 6));
    }
}
