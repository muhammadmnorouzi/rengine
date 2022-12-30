pub struct WindowSettings<'a> {
    title: &'a str,
    width: u32,
    height: u32,
}

impl<'a> WindowSettings<'a> {
    pub fn new(title: &'a str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
        }
    }

    pub fn get_title(&self) -> &'a str {
        return self.title;
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }
}
