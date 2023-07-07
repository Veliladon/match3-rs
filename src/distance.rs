use bevy::math::UVec2;

pub trait LDistance {
    fn ldistance(&self, rhs: UVec2) -> u64;
}

pub trait CDistance {
    fn cdistance(&self, rhs: UVec2) -> u64;
}

impl LDistance for UVec2 {
    fn ldistance(&self, rhs: UVec2) -> u64 {
        (self.x.abs_diff(rhs.x)) as u64 + (self.y.abs_diff(rhs.y) as u64)
    }
}

impl CDistance for UVec2 {
    fn cdistance(&self, rhs: UVec2) -> u64 {
        std::cmp::max(
            (self.x.abs_diff(rhs.x)) as u64,
            (self.y.abs_diff(rhs.y)) as u64,
        )
    }
}
