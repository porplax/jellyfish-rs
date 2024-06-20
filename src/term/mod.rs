pub struct CalculationOption {
    pub disable_color_operations: bool,
}

impl CalculationOption {
    pub fn new(disable_color_operations: bool) -> CalculationOption {
        CalculationOption {
            disable_color_operations,
        }
    }
}