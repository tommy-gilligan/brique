#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Case {
    Upper,
    Lower,
    Number,
}

#[derive(Debug)]
pub struct CaseState {
    case: Case,
    prev_case: Case,
}

impl CaseState {
    pub fn new(case: Case) -> Self {
        Self {
            case,
            prev_case: case,
        }
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn enable_numeric_case(&mut self) {
        self.prev_case = self.case;
        self.case = Case::Number;
    }

    pub fn cycle_case(&mut self) {
        self.case = match self.case {
            Case::Number => match self.prev_case {
                Case::Upper => Case::Lower,
                Case::Lower => Case::Upper,
                Case::Number => Case::Upper,
            },
            Case::Upper => Case::Lower,
            Case::Lower => Case::Upper,
        }
    }
}
