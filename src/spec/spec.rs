use crate::core::core::ReplaceRuleInterface;

pub struct CyclicnumberRule {
    pub base: u32,
    pub replacement: String,
}

pub struct PassThroughRule {}

// 割り切れたとき
impl ReplaceRuleInterface for CyclicnumberRule {
    fn fb_match(&self, _carry: String, n: u32) -> bool {
        let match_flag: bool = n % self.base == 0;
        match_flag
    }
    // carryは前の結果のFizzを持っている可能性がある。
    // 最初に引数に入れた文字列はreplacementに入っていて、
    // carryは計算結果の文字列が入る。carryが一生空なんてことにはならない
    // coreでresultにcarryの結果を入れてそれを引数に入れているので、carryはfizzruleとbuzzruleの間で別の値にならない
    // resultが初期化されるとcarryも初期化される。
    fn apply(&self, carry: String, _n: u32) -> String {
        let fb_string: String = carry + &self.replacement;
        fb_string
    }
}

// 割り切れなかった時
impl ReplaceRuleInterface for PassThroughRule {
    fn fb_match(&self, carry: String, _n: u32) -> bool {
        carry == ""
    }
    fn apply(&self, _carry: String, n: u32) -> String {
        n.to_string()
    }
}
