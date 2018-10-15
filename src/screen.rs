pub struct Screen
{
    pub width: usize,
    pub height: usize
}

impl Screen
{
    pub fn aspect(&self) -> f32
    {
        (self.width as f32)/(self.height as f32)
    }
}