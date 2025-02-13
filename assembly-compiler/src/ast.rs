
//#[derive(Debug, FromPest)]
pub struct Program {
    pub symbols: Vec<u8>
}

pub struct Alignment {
    pub alignement: usize,
}

pub enum Directive {
    Alignment(Alignment),
    Other(String),
}