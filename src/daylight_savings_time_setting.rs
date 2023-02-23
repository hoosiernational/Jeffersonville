pub enum DaylightSavingsTimeSetting {
    None,
    American,
    Cuban,
    European,
    Moldovan,
    Lebanese,
    Israeli,
    Palestinian,
    Chilean,
    Paraguayan,
    Australian,
    LordHowe,
    Kiwi,
    Troll,
    Casey,
    Moroccan,
}

impl DaylightSavingsTimeSetting {
    pub fn summer_offset_adjustment(&self) -> i16 {
        match self {
            Self::None => 0,
            Self::American => 60,
            Self::Cuban => 60,
            Self::European => 60,
            Self::Moldovan => 60,
            Self::Lebanese => 60,
            Self::Israeli => 60,
            Self::Palestinian => 60,
            Self::Chilean => -60,
            Self::Paraguayan => -60,
            Self::Australian => -60,
            Self::LordHowe => -30,
            Self::Kiwi => -60,
            Self::Troll => 120,  //European Dates
            Self::Casey => -180, //Kiwi Dates
            Self::Moroccan => -60,
        }
    }
}