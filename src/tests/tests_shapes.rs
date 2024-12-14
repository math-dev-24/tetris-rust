use crate::types::static_shape::{SCUBE, SL, SLINE, SZ};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_cube() {
        let mut cube = SCUBE.clone();
        cube.rotate();
        let cube2 = vec![(0, 0), (0, -1), (1, 0), (1, -1)];
        assert_eq!(cube.blocks, cube2);
    }

    #[test]
    fn test_rotate_line_1() {
        let mut line = SLINE.clone();
        line.rotate();
        let line2 = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        assert_eq!(line.blocks, line2);
    }

    #[test]
    fn test_rotate_line_2() {
        let mut line = SLINE.clone();
        line.rotate();
        line.rotate();
        let line2 = vec![(0, 0), (0, -1), (0, -2), (0, -3)];
        assert_eq!(line.blocks, line2);
    }

    #[test]
    fn test_rotate_l() {
        let mut l_shape = SL.clone();
        l_shape.rotate();
        let expected = vec![(0, 0), (0, -1), (1, -1)];
        assert_eq!(l_shape.blocks, expected);
    }

    #[test]
    fn test_rotate_l2() {
        let mut l_shape = SL.clone();
        l_shape.rotate();
        l_shape.rotate();
        let expected = vec![(0, 0), (-1, 0), (-1, -1)];
    }

    #[test]
    fn test_rotate_z() {
        let mut z_shape = SZ.clone();
        z_shape.rotate();
        let expected = vec![(0, 0), (0, -1), (1, -1), (1, -2)];
        assert_eq!(z_shape.blocks, expected);
    }

    #[test]
    fn test_rotate_z2() {
        let mut z_shape = SZ.clone();
        z_shape.rotate();
        z_shape.rotate();
        let expected = vec![(0, 0), (-1, 0), (-1, -1), (-2, -1)];
        assert_eq!(z_shape.blocks, expected);
    }
}