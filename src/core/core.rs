pub trait ReplaceRuleInterface {
    fn fb_match(&self, carry: String, n: u32) -> bool;
    fn apply(&self, carry: String, n: u32) -> String;
}

pub struct NumberConverter {
    pub rules: Vec<Box<dyn ReplaceRuleInterface>>,
}

impl NumberConverter {
    pub fn convert(&self, n: u32) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        for i in 1..n + 1 {
            let mut result: String = String::new();
            for rule in self.rules.iter() {
                if rule.fb_match(result.clone(), i) {
                    result = rule.apply(result, i)
                }
            }
            vec.push(result);
        }
        vec
    }
}
