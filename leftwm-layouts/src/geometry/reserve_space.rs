#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReserveColumnSpace {
    None,
    Reserve,
    ReserveAndCenter,
}

impl ReserveColumnSpace {
    pub fn is_reserved(&self) -> bool {
        match self {
            ReserveColumnSpace::None => false,
            ReserveColumnSpace::Reserve | ReserveColumnSpace::ReserveAndCenter => true,
        }
    }
}

impl Default for ReserveColumnSpace {
    fn default() -> Self {
        ReserveColumnSpace::None
    }
}

#[cfg(test)]
mod tests {}
