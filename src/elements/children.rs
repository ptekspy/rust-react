pub struct Empty;

pub struct More<H, T> {
    pub head: H,
    pub tail: T,
}