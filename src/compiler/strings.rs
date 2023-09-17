#[derive(Debug, Clone)]
pub struct StringsContainer {
    strings: Vec<String>,
}

impl StringsContainer {
    pub fn new() -> Self {
        Self { strings: Vec::new() }
    }

    pub fn register_string(&mut self, string: String) -> u64 {
        if self.strings.contains(&string) {
            return self.strings.iter().position(|string2| &string == string2).unwrap() as u64;
        }
        self.strings.push(string);
        (self.strings.len() - 1) as u64
    }

    pub fn get_strings(&self) -> Vec<String> {
        self.strings.clone()
    }
}
