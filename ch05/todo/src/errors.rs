// 独自のエラーを定義
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("何らかのエラーが発生しました")]
    SomethingWentWrong,
}

// MyError から loco_rs::Error への変換
impl From<MyError> for loco_rs::Error {
    fn from(err: MyError) -> Self {
        match err {
            MyError::SomethingWentWrong => loco_rs::Error::string("何らかのエラーが発生しました"),
        }
    }
}
