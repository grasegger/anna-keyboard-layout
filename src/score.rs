#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub points: u128,
    pub x1: f32,
    pub x2: f32,
    pub x3: f32,
    pub x4: f32,
    pub x5: f32,
    pub inward_rolls: f32,
    pub outward_rolls: f32,
    pub top_10_chars_on_home: f32,
    pub space_on_thumb: f32,
    pub inward_rolls_left: f32,
    pub inward_rolls_right: f32,
    pub counter_left: u128,
    pub counter_right: u128,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            points: 0,
            x1: 0.0,
            x2: 0.0,
            x3: 0.0,
            x4: 0.0,
            x5: 0.0,
            inward_rolls: 0.0,
            outward_rolls: 0.0,
            top_10_chars_on_home: 0.0,
            space_on_thumb: 0.0,
            counter_left: 0,
            counter_right: 0,
            inward_rolls_left: 0.0,
            inward_rolls_right: 0.0,
        }
    }
}

impl Eq for Score {}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.points.partial_cmp(&other.points) {
            Some(ord) => ord,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.points.partial_cmp(&other.points)
    }
}
