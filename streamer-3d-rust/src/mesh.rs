pub struct InputMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[usize; 3]>,
    // Normals will be added one day.
}
impl InputMesh {
    pub fn new () -> InputMesh {
        InputMesh {
            vertices : Vec::<[f32; 3]>::new(),
            triangles : Vec::<[usize; 3]>::new(),
        }
    }
}