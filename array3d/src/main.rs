
fn main() {

    use multiarray::*;
    let mut voxels = Array3D::new([3,4,5], 0); // 3x4x5 ints
    voxels[[0,0,0]] = 1;
    voxels[[1,2,3]] = 23;
    voxels[[2,3,4]] = 42;
    assert!(voxels[[1,2,3]] == 23);
    let slice = voxels.eliminated_dim(1, 2);   // 2D slice
    assert!(slice[[1,3]] == 23);
    let lane = slice.eliminated_dim(1, 3);     // 1D lane
    assert!(lane[1] == 23);
    // println!(voxels);
}
