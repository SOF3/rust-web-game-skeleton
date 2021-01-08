#[derive(codegen::Gen)]
pub struct Login {
    pub identity: [u8; 512 / 8],
    pub name: String,
}

#[derive(codegen::Gen)]
pub struct Accept {}

#[derive(codegen::Gen)]
pub struct Reject {
    pub reason: String,
}
