


#[derive(Debug)]
pub struct Response<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}