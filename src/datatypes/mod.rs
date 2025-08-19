pub struct ElementUnit {
    pub name: String,
    exp: f64,
    pub dim: Option<String>,
    conversionfactor: f64,
}

impl ElementUnit {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            exp: 0.,
            dim: None,
            conversionfactor: 0.,
        }
    }

    pub fn set_dim(&mut self, dim: &str) {
        self.dim = Some(dim.to_owned());
    }
}

pub struct Unit {
    pub partials: Vec<ElementUnit>,
}

impl Unit {
    fn from_v(partials: Vec<ElementUnit>) -> Self {
        Self { partials }
    }
}

impl From<ElementUnit> for Unit {
    fn from(val: ElementUnit) -> Self {
        Unit {
            partials: vec![val],
        }
    }
}

pub struct Value {
    unit: Unit,
    value: f64,
}

impl Value {
    fn new(unit: Unit) -> Self {
        Self { unit, value: 0. }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Dimension(pub [i32; 7]);
