


#[derive(PartialEq, Clone, Debug)]
pub struct Fluid{
    /// Unique identifier to this chemical. Expressed in InChI standard version 1.
    /// 
    /// While the identifier could be theoretically any string, InChI standard notation is used to prevent the creation of duplicate chemicals
    /// 
    /// EG: Pure Water = InChI=1S/H2O/h1H2
    pub identifier : String,
}

impl Fluid {
    fn pure_water() -> Self{
        Fluid { 
            identifier: "pure_water".to_string()
        }
    }
}

impl Default for Fluid{
    fn default() -> Self {
        Fluid::pure_water()
    }
}