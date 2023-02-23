use crate::unix_timestamp::{UnixTimestamp};

#[derive(Debug)]
pub struct ModularTimestamp {
    quadcenturies_since_march_1601: u64,
    centuries_since_quadcentury_start: u8,
    quadyears_since_century_start: u8,
    years_since_quadyear_start: u8,
    days_since_year_start: u16,
    hour: u8,
    minute: u8,
    second: u8,
    weekday: u8,
}

impl ModularTimestamp {
    fn seconds_after_march_1601(unix_timestamp: &UnixTimestamp) -> u64 {
        unix_timestamp.as_u64() + 11639376000
    }

    fn unix_timestamp_from_seconds_after_march_1601(input:u64) -> UnixTimestamp {
        UnixTimestamp::from_u64(input - 11639376000)
    }

    pub fn gen(
        quadcenturies_since_march_1601: u64,
        centuries_since_quadcentury_start: u8,
        quadyears_since_century_start: u8,
        years_since_quadyear_start: u8,
        days_since_year_start: u16,
        hour: u8,
        minute: u8,
        second: u8,
        weekday: u8,
    ) -> Self {
        Self {
            quadcenturies_since_march_1601,
            centuries_since_quadcentury_start,
            quadyears_since_century_start,
            years_since_quadyear_start,
            days_since_year_start,
            hour,
            minute,
            second,
            weekday,
        }
    }

    pub fn to_unix_timestamp(&self) -> UnixTimestamp {
        Self::unix_timestamp_from_seconds_after_march_1601(
            ((self.quadcenturies_since_march_1601 as u64) * 12_622_780_800_u64)
                + ((self.centuries_since_quadcentury_start as u64) * 3_155_673_600)
                + ((self.quadyears_since_century_start as u64) * 126_230_400)
                + ((self.years_since_quadyear_start as u64) * 31_536_000)
                + ((self.days_since_year_start as u64) * 86_400)
                + ((self.hour as u64) * 3_600)
                + ((self.minute as u64) * 60)
                + (self.second as u64),
        )
    }

    pub fn from_unix_timestamp(input: &UnixTimestamp) -> Self {
        let mut seconds_after_march_1601 = Self::seconds_after_march_1601(input);
        let weekday = (((seconds_after_march_1601 / 86400) + 4) % 7) as u8;
        let quadcenturies_since_march_1601 = seconds_after_march_1601 / 12_622_780_800_u64;
        seconds_after_march_1601 %= 12_622_780_800_u64;
        let centuries_since_quadcentury_start = (seconds_after_march_1601 / 3_155_673_600) as u8;
        seconds_after_march_1601 %= 3_155_673_600;
        let quadyears_since_century_start = (seconds_after_march_1601 / 126_230_400) as u8;
        seconds_after_march_1601 %= 126_230_400;
        let years_since_quadyear_start = (seconds_after_march_1601 / 31_536_000) as u8;
        seconds_after_march_1601 %= 31_536_000;
        let days_since_year_start = (seconds_after_march_1601 / 86_400) as u16;
        seconds_after_march_1601 %= 86_400;
        let hour = (seconds_after_march_1601 / 3_600) as u8;
        seconds_after_march_1601 %= 3_600;
        let minute = (seconds_after_march_1601 / 60) as u8;
        Self {
            quadcenturies_since_march_1601,
            centuries_since_quadcentury_start,
            quadyears_since_century_start,
            years_since_quadyear_start,
            days_since_year_start,
            hour,
            minute,
            second: (seconds_after_march_1601 % 60) as u8,
            weekday,
        }
    }

    fn year_weekday_offset(&self) -> usize {
        (371 + (self.weekday as usize) - (self.days_since_year_start as usize)) % 7
    }

    fn utc_year_minute(&self) -> i32 {
        ((self.days_since_year_start as i32) * 1440)
            + ((self.hour as i32) * 60)
            + (self.minute as i32)
    }

    pub fn is_american_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [10200, 18840, 17400, 15960, 14520, 13080, 11640][year_weekday_offset]
            && year_minute
                < [352860, 361500, 360060, 358620, 357180, 355740, 354300][year_weekday_offset]
    }

    pub fn is_cuban_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [10080, 18720, 17280, 15840, 14400, 12960, 11520][year_weekday_offset]
            && year_minute < [358560, 357120, 355680, 352800, 361440, 360000][year_weekday_offset] //TODO HP: THIS IS WRONG; RESTART THIS LINE
    }

    pub fn is_european_summer_time(&self) -> bool {
        let year_minute = self.utc_year_minute();
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [40380, 38940, 37500, 36060, 34620, 43260, 41820][year_weekday_offset]
            && year_minute
                < [342780, 351420, 349980, 348540, 347100, 345660, 344220][year_weekday_offset]
    }

    pub fn is_moldovan_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [40440, 39000, 37560, 36120, 34680, 43320, 41880][year_weekday_offset]
            && year_minute
                < [342840, 351480, 350040, 348600, 347160, 345720, 344280][year_weekday_offset]
    }

    pub fn is_lebanese_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [40320, 38880, 37440, 36000, 34560, 43200, 41760][year_weekday_offset]
            && year_minute
                < [342660, 351300, 349860, 348420, 346980, 345540, 344100][year_weekday_offset]
    }
    pub fn is_israeli_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [37560, 36120, 34680, 33240, 31800, 40440, 39000][year_weekday_offset]
            && year_minute
                < [342780, 351420, 349980, 348540, 347100, 345660, 344220][year_weekday_offset]
    }
    pub fn is_palestinian_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [39000, 37560, 36120, 34680, 33240, 41880, 40440][year_weekday_offset]
            && year_minute
                < [341340, 349980, 348540, 347100, 345660, 344220, 342780][year_weekday_offset]
    }
    pub fn is_chilean_summer_time(&self) -> bool {
        let year_minute = self.utc_year_minute();
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [50580, 49140, 47700, 46260, 44820, 53460, 52020][year_weekday_offset]
            && year_minute
                < [283920, 282480, 281040, 279600, 278160, 276720, 285360][year_weekday_offset]
    }
    pub fn is_paraguayan_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [30180, 38820, 37380, 35940, 34500, 33060, 31620][year_weekday_offset]
            && year_minute
                < [312480, 311040, 309600, 308160, 316800, 315360, 313920][year_weekday_offset]
    }
    pub fn is_australian_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [50520, 49080, 47640, 46200, 44760, 53400, 51960][year_weekday_offset]
            && year_minute
                < [312600, 311160, 309720, 308280, 316920, 315480, 314040][year_weekday_offset]
    }
    pub fn is_lord_howe_summer_time(&self, offset_minutes: i32) -> bool {
        let year_minute = self.utc_year_minute() + offset_minutes;
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [50460, 49020, 47580, 46140, 44700, 53340, 51900][year_weekday_offset]
            && year_minute
                < [312600, 311160, 309720, 308280, 316920, 315480, 314040][year_weekday_offset]
    }
    pub fn is_kiwi_summer_time(&self) -> bool {
        let year_minute = self.utc_year_minute();
        let year_weekday_offset = self.year_weekday_offset();
        year_minute >= [49740, 48300, 46860, 45420, 43980, 52620, 51180][year_weekday_offset]
            && year_minute
                < [301800, 300360, 298920, 297480, 306120, 304680, 303240][year_weekday_offset]
    }

    pub fn is_moroccan_summer_time(&self, _offset_minutes: i32) -> bool {
        //TODO
        false
    }

    pub fn actual_utc_year(&self) -> u64 {
        if self.years_since_quadyear_start == 4 {
            (self.years_since_quadyear_start as u64) +
            ((self.quadyears_since_century_start as u64) * 4) + 
            ((self.centuries_since_quadcentury_start as u64) * 100) + 
            (self.quadcenturies_since_march_1601 * 400)
            + 1601
        } else if self.days_since_year_start >= 305 {
            (self.years_since_quadyear_start as u64) +
            ((self.quadyears_since_century_start as u64) * 4) + 
            ((self.centuries_since_quadcentury_start as u64) * 100) + 
            (self.quadcenturies_since_march_1601 * 400)
            + 1602
        } else {
            (self.years_since_quadyear_start as u64) +
            ((self.quadyears_since_century_start as u64) * 4) + 
            ((self.centuries_since_quadcentury_start as u64) * 100) + 
            (self.quadcenturies_since_march_1601 * 400)
            + 1601
        }
    }

    pub fn actual_utc_month_and_date(&self) -> (u8,u8) {
        if self.years_since_quadyear_start == 4 {
            (2,29)
        } else {
            let mut month = 0_u8;
            let mut date = 0_u8;
            let mod_days = self.days_since_year_start;
            let days = [[0,30],
            [31,60],
            [61,91],
            [92,121],
            [122,152],
            [153,183],
            [184,213],
            [214,244],
            [245,274],
            [275,305],
            [306,336],
            [337,364]];
            for i in 0..12 {
                if days[i][0] <= mod_days && days[i][1] >= mod_days {
                    month = (((i as u8) + 2) % 12) + 1;
                    date = ((mod_days - days[i][0]) + 1) as u8;
                }
            }
            (month,date)
        }
    }

    pub fn get_hour(&self) -> u8 {
        self.hour
    }

    pub fn get_minute(&self) -> u8 {
        self.minute
    }

    pub fn get_second(&self) -> u8 {
        self.second
    }

    pub fn get_weekday(&self) -> u8 {
        self.weekday
    }
}
