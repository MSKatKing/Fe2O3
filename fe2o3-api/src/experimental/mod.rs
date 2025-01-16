use shipyard::Unique;

#[derive(Unique)]
pub struct ExperimentalStruct {
    pub added_by: String,
}

pub struct Teo;