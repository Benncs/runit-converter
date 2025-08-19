pub struct ElementUnit {
    pub name: String,
    exp: f64,
    pub dim: Option<String>,
    conversionfactor: f64,
}

impl ElementUnit {
    pub fn new(name: &str, exp: f64) -> Self {
        Self {
            name: name.to_owned(),
            exp,
            dim: None,
            conversionfactor: 0.,
        }
    }
    pub fn exp(&self) -> f64 {
        self.exp
    }

    pub fn set_exp(&mut self, exp: f64) {
        self.exp = exp;
    }

    pub fn set_dim(&mut self, dim: &str) {
        self.dim = Some(dim.to_owned());
    }
}

pub struct Unit {
    pub partials: Vec<ElementUnit>,
}

impl Unit {
    pub fn from_vec(partials: Vec<ElementUnit>) -> Self {
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

#[derive(Default, Clone, Copy, Debug)]
pub struct Dimension(pub [i32; 7]);

impl Dimension {
    pub fn dot(&self, val: &Self, exp: f64) -> Self {
        let e: [i32; 7] = self
            .0
            .iter()
            .zip(val.0.iter())
            .map(|(di, dj)| *di + ((*dj as f64) * exp) as i32)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self(e)
    }
}

impl PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(d1, d2)| d1 == d2)
    }
}

impl Eq for Dimension {}
