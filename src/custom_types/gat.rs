//
// Generic Associated Types (GATs)
//
// -- Generic Associated Types allow for even more powerful abstractions 
// by making associated types generic over parameters:

// Define a trait for streaming operations
pub trait Stream {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
    
    // 새로운 메소드: 아이템과 position을 함께 반환
    fn next_with_position<'a>(&'a mut self) -> Option<(Self::Item<'a>, usize)>
    where
        Self: Sized;

    fn reset_position(&mut self) -> &mut Self;
}

// Example implementation for a string stream
#[derive(Debug, Clone)]
pub struct StringStream {
    pub data: String,
    pub position: usize,
}

impl Stream for StringStream {
    type Item<'a> = &'a str 
    where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position >= self.data.len() {
            return None;
        }

        let slice = &self.data[self.position..];
        let word_end = slice.find(' ').unwrap_or(slice.len());
        let result = &slice[..word_end];
        self.position += word_end + 1;
        Some(result)
    }

    fn next_with_position<'a>(&'a mut self) -> Option<(Self::Item<'a>, usize)>
    where
        Self: Sized
    {
        if self.position >= self.data.len() {
            return None;
        }

        let slice = &self.data[self.position..];
        let word_end = slice.find(' ').unwrap_or(slice.len());
        let result = &slice[..word_end];
        let position = self.position;
        self.position += word_end + 1;
        Some((result, position))
    }

    fn reset_position(&mut self) -> &mut Self {
        self.position = 0;
        self
    }
}

// Example implementation for an integer stream
pub struct IntStream {
    pub data: Vec<i32>,
    pub position: usize,
}

impl Stream for IntStream {
    type Item<'a> = &'a i32
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position >= self.data.len() {
            return None;
        }

        let result = &self.data[self.position];
        self.position += 1;
        Some(result)
    }

    fn next_with_position<'a>(&'a mut self) -> Option<(Self::Item<'a>, usize)>
    where
        Self: Sized
    {
        if self.position >= self.data.len() {
            return None;
        }

        let result = &self.data[self.position];
        let position = self.position;
        self.position += 1;
        Some((result, position))
    }

    fn reset_position(&mut self) -> &mut Self {
        self.position = 0;
        self
    }
}