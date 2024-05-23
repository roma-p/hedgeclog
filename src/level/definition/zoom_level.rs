
pub enum ZoomLevel {
    REALLYSMALL = 6,
    SMALL = 10,
    NORMAL = 12,
    BIG = 15,
    REALLYBIG = 20,
}

impl ZoomLevel {
    pub fn unzoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::REALLYBIG),
            ZoomLevel::REALLYBIG => Some(ZoomLevel::REALLYBIG),
        }
    }

    pub fn zoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYBIG => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::REALLYSMALL),
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
        }
    }

    pub fn get_from_i32(value: i32) -> Option<ZoomLevel> {

        const I32_REALLYSMALL: i32 = ZoomLevel::REALLYSMALL as i32;
        const I32_SMALL: i32 = ZoomLevel::SMALL as i32;
        const I32_NORMAL: i32 = ZoomLevel::NORMAL as i32;
        const I32_BIG: i32 = ZoomLevel::BIG as i32;
        const I32_REALLYBIG: i32 = ZoomLevel::REALLYBIG as i32;

        match value {
            I32_REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
            I32_SMALL => Some(ZoomLevel::SMALL),
            I32_NORMAL=> Some(ZoomLevel::NORMAL),
            I32_BIG => Some(ZoomLevel::BIG),
            I32_REALLYBIG => Some(ZoomLevel::REALLYBIG),
            _ => None
        }
    }
}

