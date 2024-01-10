#[allow(dead_code)]
struct Book {
    pub id: String,
    pub name: String,
}
trait SchoolDef {
    type Output;
    fn get_school_name() -> String;
    fn get_school_location() -> String;
    fn return_self() -> Self::Output;
}
pub enum LearningTips {
    Focus,
    SetMinimumTime,
    AllocateBreak,
    SetReward,
}

impl LearningTips {
    pub fn get_id(&self) -> u16 {
        match self {
            LearningTips::Focus => 1,
            LearningTips::AllocateBreak => 2,
            LearningTips::SetReward => 3,
            LearningTips::SetMinimumTime => 4,
        }
    }
}

impl SchoolDef for LearningTips {
    type Output = Book;

    fn get_school_name() -> String {
        todo!()
    }

    fn get_school_location() -> String {
        todo!()
    }

    fn return_self() -> Book {
        todo!()
    }
}


fn main() {
    let v = LearningTips::AllocateBreak;

    v.get_id();
}