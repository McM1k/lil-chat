
pub struct User {
    pub nickname: String,
}

impl User {
    pub fn new(nickname: String) -> User {
        User {
            nickname,
        }
    }

    pub fn change_nickname(&mut self, new_nickname: String) {
        self.nickname = new_nickname;
    }
}