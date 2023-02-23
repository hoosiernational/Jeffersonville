use crate::{modular_timestamp::ModularTimestamp, Timezone, UnixTimestamp, daylight_savings_time_setting::DaylightSavingsTimeSetting};

///A detailed timestamp generated from a UNIX Timestamp and a Timezone
#[derive(Debug)]
pub struct LocalTimestamp {
    ///The actual Gregorian year (e.g. 2023)
    pub year: u64,

    ///The month (Jan = 1, Dec = 12)
    pub month: u8,

    ///The date within the month ("14" for April 14)
    pub date: u8,

    ///The hour within the day (0 at midnight, 23 at 11 PM)
    pub hour: u8,

    ///The minute
    pub minute: u8,

    ///The second
    pub second: u8,

    ///The day of the week (0 = Sunday, 6 = Saturday).
    pub weekday: u8,
}

impl LocalTimestamp {
    fn offset_minutes_and_dst(timezone: &Timezone) -> (i16, DaylightSavingsTimeSetting) {
        match timezone.get_u16() {
            1 => (-300, DaylightSavingsTimeSetting::American),
            2 => (-360, DaylightSavingsTimeSetting::American),
            3 => (-420, DaylightSavingsTimeSetting::None),
            4 => (-420, DaylightSavingsTimeSetting::American),
            5 => (-480, DaylightSavingsTimeSetting::American),
            6 => (-540, DaylightSavingsTimeSetting::American),
            7 => (-600, DaylightSavingsTimeSetting::None),
            8 => (-600, DaylightSavingsTimeSetting::American),
            9 => (-240, DaylightSavingsTimeSetting::None),
            10 => (-660, DaylightSavingsTimeSetting::None),
            11 => (600, DaylightSavingsTimeSetting::None),
            12 => (-180, DaylightSavingsTimeSetting::None),
            13 => (270, DaylightSavingsTimeSetting::None),
            14 => (300, DaylightSavingsTimeSetting::None),
            15 => (60, DaylightSavingsTimeSetting::European),
            16 => (660, DaylightSavingsTimeSetting::None),
            17 => (60, DaylightSavingsTimeSetting::None),
            18 => (60, DaylightSavingsTimeSetting::European),
            19 => (60, DaylightSavingsTimeSetting::European),
            20 => (-240, DaylightSavingsTimeSetting::None),
            21 => (60, DaylightSavingsTimeSetting::None),
            22 => (420, DaylightSavingsTimeSetting::None),
            23 => (480, DaylightSavingsTimeSetting::Casey),
            24 => (180, DaylightSavingsTimeSetting::None),
            25 => (420, DaylightSavingsTimeSetting::None),
            26 => (120, DaylightSavingsTimeSetting::None),
            27 => (600, DaylightSavingsTimeSetting::Australian),
            28 => (120, DaylightSavingsTimeSetting::None),
            29 => (300, DaylightSavingsTimeSetting::None),
            30 => (720, DaylightSavingsTimeSetting::Kiwi),
            31 => (-180, DaylightSavingsTimeSetting::None),
            32 => (180, DaylightSavingsTimeSetting::None),
            33 => (0, DaylightSavingsTimeSetting::Troll),
            34 => (360, DaylightSavingsTimeSetting::None),
            35 => (-240, DaylightSavingsTimeSetting::None),
            36 => (-180, DaylightSavingsTimeSetting::None),
            37 => (240, DaylightSavingsTimeSetting::None),
            38 => (630, DaylightSavingsTimeSetting::Australian),
            39 => (570, DaylightSavingsTimeSetting::None),
            40 => (507, DaylightSavingsTimeSetting::None),
            41 => (420, DaylightSavingsTimeSetting::None),
            42 => (390, DaylightSavingsTimeSetting::None),
            43 => (660, DaylightSavingsTimeSetting::Australian),
            44 => (600, DaylightSavingsTimeSetting::None),
            45 => (300, DaylightSavingsTimeSetting::None),
            46 => (630, DaylightSavingsTimeSetting::LordHowe),
            47 => (660, DaylightSavingsTimeSetting::Australian),
            48 => (540, DaylightSavingsTimeSetting::None),
            49 => (480, DaylightSavingsTimeSetting::None),
            50 => (60, DaylightSavingsTimeSetting::None),
            51 => (240, DaylightSavingsTimeSetting::None),
            52 => (-300, DaylightSavingsTimeSetting::American),
            53 => (180, DaylightSavingsTimeSetting::None),
            54 => (360, DaylightSavingsTimeSetting::None),
            55 => (-240, DaylightSavingsTimeSetting::None),
            56 => (180, DaylightSavingsTimeSetting::None),
            57 => (60, DaylightSavingsTimeSetting::None),
            58 => (-360, DaylightSavingsTimeSetting::None),
            59 => (60, DaylightSavingsTimeSetting::None),
            60 => (360, DaylightSavingsTimeSetting::None),
            61 => (-240, DaylightSavingsTimeSetting::None),
            62 => (60, DaylightSavingsTimeSetting::European),
            63 => (120, DaylightSavingsTimeSetting::None),
            64 => (-300, DaylightSavingsTimeSetting::None),
            65 => (-240, DaylightSavingsTimeSetting::None),
            66 => (-180, DaylightSavingsTimeSetting::None),
            67 => (-120, DaylightSavingsTimeSetting::None),
            68 => (480, DaylightSavingsTimeSetting::None),
            69 => (120, DaylightSavingsTimeSetting::None),
            70 => (0, DaylightSavingsTimeSetting::None),
            71 => (120, DaylightSavingsTimeSetting::None),
            72 => (420, DaylightSavingsTimeSetting::None),
            73 => (60, DaylightSavingsTimeSetting::None),
            74 => (-240, DaylightSavingsTimeSetting::American),
            75 => (-240, DaylightSavingsTimeSetting::None),
            76 => (-360, DaylightSavingsTimeSetting::American),
            77 => (-360, DaylightSavingsTimeSetting::None),
            78 => (-300, DaylightSavingsTimeSetting::American),
            79 => (-300, DaylightSavingsTimeSetting::None),
            80 => (-420, DaylightSavingsTimeSetting::American),
            81 => (-420, DaylightSavingsTimeSetting::None),
            82 => (-210, DaylightSavingsTimeSetting::American),
            83 => (-480, DaylightSavingsTimeSetting::American),
            84 => (-60, DaylightSavingsTimeSetting::None),
            85 => (60, DaylightSavingsTimeSetting::None),
            86 => (60, DaylightSavingsTimeSetting::None),
            87 => (-360, DaylightSavingsTimeSetting::Chilean),
            88 => (-180, DaylightSavingsTimeSetting::None),
            89 => (-180, DaylightSavingsTimeSetting::Chilean),
            90 => (480, DaylightSavingsTimeSetting::None),
            91 => (-300, DaylightSavingsTimeSetting::None),
            92 => (180, DaylightSavingsTimeSetting::None),
            93 => (120, DaylightSavingsTimeSetting::None),
            94 => (60, DaylightSavingsTimeSetting::None),
            95 => (-360, DaylightSavingsTimeSetting::None),
            96 => (60, DaylightSavingsTimeSetting::None),
            97 => (-300, DaylightSavingsTimeSetting::Cuban),
            98 => (120, DaylightSavingsTimeSetting::None),
            99 => (60, DaylightSavingsTimeSetting::None),
            100 => (60, DaylightSavingsTimeSetting::European),
            101 => (0, DaylightSavingsTimeSetting::None),
            102 => (0, DaylightSavingsTimeSetting::European),
            103 => (-60, DaylightSavingsTimeSetting::European),
            104 => (-240, DaylightSavingsTimeSetting::American),
            105 => (-180, DaylightSavingsTimeSetting::European),
            106 => (180, DaylightSavingsTimeSetting::None),
            107 => (-240, DaylightSavingsTimeSetting::None),
            108 => (-240, DaylightSavingsTimeSetting::None),
            109 => (540, DaylightSavingsTimeSetting::None),
            110 => (-300, DaylightSavingsTimeSetting::None),
            111 => (-360, DaylightSavingsTimeSetting::None),
            112 => (120, DaylightSavingsTimeSetting::None),
            113 => (-360, DaylightSavingsTimeSetting::None),
            114 => (60, DaylightSavingsTimeSetting::None),
            115 => (180, DaylightSavingsTimeSetting::None),
            116 => (120, DaylightSavingsTimeSetting::None),
            117 => (120, DaylightSavingsTimeSetting::None),
            118 => (180, DaylightSavingsTimeSetting::None),
            119 => (720, DaylightSavingsTimeSetting::None),
            120 => (120, DaylightSavingsTimeSetting::None),
            121 => (-240, DaylightSavingsTimeSetting::None),
            122 => (-480, DaylightSavingsTimeSetting::None),
            123 => (-180, DaylightSavingsTimeSetting::None),
            124 => (-540, DaylightSavingsTimeSetting::None),
            125 => (300, DaylightSavingsTimeSetting::None),
            126 => (-570, DaylightSavingsTimeSetting::None),
            127 => (180, DaylightSavingsTimeSetting::None),
            128 => (60, DaylightSavingsTimeSetting::European),
            129 => (660, DaylightSavingsTimeSetting::None),
            130 => (240, DaylightSavingsTimeSetting::None),
            131 => (-180, DaylightSavingsTimeSetting::American),
            132 => (-600, DaylightSavingsTimeSetting::None),
            133 => (720, DaylightSavingsTimeSetting::None),
            134 => (60, DaylightSavingsTimeSetting::None),
            135 => (0, DaylightSavingsTimeSetting::None),
            136 => (240, DaylightSavingsTimeSetting::None),
            137 => (60, DaylightSavingsTimeSetting::None),
            138 => (0, DaylightSavingsTimeSetting::None),
            139 => (120, DaylightSavingsTimeSetting::None),
            140 => (-240, DaylightSavingsTimeSetting::None),
            141 => (-360, DaylightSavingsTimeSetting::None),
            142 => (0, DaylightSavingsTimeSetting::None),
            143 => (0, DaylightSavingsTimeSetting::None),
            144 => (-240, DaylightSavingsTimeSetting::None),
            145 => (-300, DaylightSavingsTimeSetting::American),
            146 => (60, DaylightSavingsTimeSetting::None),
            147 => (120, DaylightSavingsTimeSetting::None),
            148 => (180, DaylightSavingsTimeSetting::None),
            149 => (240, DaylightSavingsTimeSetting::None),
            150 => (300, DaylightSavingsTimeSetting::None),
            151 => (360, DaylightSavingsTimeSetting::None),
            152 => (420, DaylightSavingsTimeSetting::None),
            153 => (480, DaylightSavingsTimeSetting::None),
            154 => (540, DaylightSavingsTimeSetting::None),
            155 => (600, DaylightSavingsTimeSetting::None),
            156 => (660, DaylightSavingsTimeSetting::None),
            157 => (720, DaylightSavingsTimeSetting::None),
            158 => (-60, DaylightSavingsTimeSetting::None),
            159 => (-120, DaylightSavingsTimeSetting::None),
            160 => (-180, DaylightSavingsTimeSetting::None),
            161 => (-240, DaylightSavingsTimeSetting::None),
            162 => (-300, DaylightSavingsTimeSetting::None),
            163 => (-360, DaylightSavingsTimeSetting::None),
            164 => (-420, DaylightSavingsTimeSetting::None),
            165 => (-480, DaylightSavingsTimeSetting::None),
            166 => (-540, DaylightSavingsTimeSetting::None),
            167 => (-600, DaylightSavingsTimeSetting::None),
            168 => (-660, DaylightSavingsTimeSetting::None),
            169 => (-720, DaylightSavingsTimeSetting::None),
            170 => (-360, DaylightSavingsTimeSetting::None),
            171 => (480, DaylightSavingsTimeSetting::None),
            172 => (60, DaylightSavingsTimeSetting::None),
            173 => (0, DaylightSavingsTimeSetting::None),
            174 => (330, DaylightSavingsTimeSetting::None),
            175 => (480, DaylightSavingsTimeSetting::None),
            176 => (540, DaylightSavingsTimeSetting::None),
            177 => (420, DaylightSavingsTimeSetting::None),
            178 => (210, DaylightSavingsTimeSetting::None),
            179 => (180, DaylightSavingsTimeSetting::None),
            180 => (0, DaylightSavingsTimeSetting::None),
            181 => (120, DaylightSavingsTimeSetting::Israeli),
            182 => (60, DaylightSavingsTimeSetting::None),
            183 => (0, DaylightSavingsTimeSetting::None),
            184 => (-300, DaylightSavingsTimeSetting::None),
            185 => (540, DaylightSavingsTimeSetting::None),
            186 => (180, DaylightSavingsTimeSetting::None),
            187 => (360, DaylightSavingsTimeSetting::None),
            188 => (300, DaylightSavingsTimeSetting::None),
            189 => (180, DaylightSavingsTimeSetting::None),
            190 => (720, DaylightSavingsTimeSetting::None),
            191 => (840, DaylightSavingsTimeSetting::None),
            192 => (780, DaylightSavingsTimeSetting::None),
            193 => (60, DaylightSavingsTimeSetting::European),
            194 => (180, DaylightSavingsTimeSetting::None),
            195 => (360, DaylightSavingsTimeSetting::None),
            196 => (420, DaylightSavingsTimeSetting::None),
            197 => (120, DaylightSavingsTimeSetting::None),
            198 => (120, DaylightSavingsTimeSetting::Lebanese),
            199 => (120, DaylightSavingsTimeSetting::None),
            200 => (0, DaylightSavingsTimeSetting::None),
            201 => (120, DaylightSavingsTimeSetting::None),
            202 => (60, DaylightSavingsTimeSetting::European),
            203 => (120, DaylightSavingsTimeSetting::None),
            204 => (60, DaylightSavingsTimeSetting::None),
            205 => (480, DaylightSavingsTimeSetting::None),
            206 => (180, DaylightSavingsTimeSetting::None),
            207 => (120, DaylightSavingsTimeSetting::None),
            208 => (480, DaylightSavingsTimeSetting::None),
            209 => (300, DaylightSavingsTimeSetting::None),
            210 => (0, DaylightSavingsTimeSetting::None),
            211 => (60, DaylightSavingsTimeSetting::None),
            212 => (720, DaylightSavingsTimeSetting::None),
            213 => (0, DaylightSavingsTimeSetting::None),
            214 => (240, DaylightSavingsTimeSetting::None),
            215 => (-360, DaylightSavingsTimeSetting::American),
            216 => (-420, DaylightSavingsTimeSetting::American),
            217 => (-480, DaylightSavingsTimeSetting::American),
            218 => (-360, DaylightSavingsTimeSetting::None),
            219 => (-420, DaylightSavingsTimeSetting::None),
            220 => (-300, DaylightSavingsTimeSetting::None),
            221 => (660, DaylightSavingsTimeSetting::None),
            222 => (600, DaylightSavingsTimeSetting::None),
            223 => (120, DaylightSavingsTimeSetting::Moldovan),
            224 => (60, DaylightSavingsTimeSetting::European),
            225 => (480, DaylightSavingsTimeSetting::None),
            226 => (420, DaylightSavingsTimeSetting::None),
            227 => (60, DaylightSavingsTimeSetting::European),
            228 => (60, DaylightSavingsTimeSetting::Moroccan),
            229 => (120, DaylightSavingsTimeSetting::None),
            230 => (390, DaylightSavingsTimeSetting::None),
            231 => (120, DaylightSavingsTimeSetting::None),
            232 => (720, DaylightSavingsTimeSetting::None),
            233 => (345, DaylightSavingsTimeSetting::None),
            234 => (-240, DaylightSavingsTimeSetting::None),
            235 => (60, DaylightSavingsTimeSetting::European),
            236 => (825, DaylightSavingsTimeSetting::Kiwi),
            237 => (-600, DaylightSavingsTimeSetting::None),
            238 => (-660, DaylightSavingsTimeSetting::None),
            239 => (780, DaylightSavingsTimeSetting::Kiwi),
            240 => (780, DaylightSavingsTimeSetting::None),
            241 => (-360, DaylightSavingsTimeSetting::None),
            242 => (60, DaylightSavingsTimeSetting::None),
            243 => (60, DaylightSavingsTimeSetting::None),
            244 => (540, DaylightSavingsTimeSetting::None),
            245 => (60, DaylightSavingsTimeSetting::European),
            246 => (60, DaylightSavingsTimeSetting::European),
            247 => (240, DaylightSavingsTimeSetting::None),
            248 => (300, DaylightSavingsTimeSetting::None),
            249 => (540, DaylightSavingsTimeSetting::None),
            250 => (120, DaylightSavingsTimeSetting::Palestinian),
            251 => (-300, DaylightSavingsTimeSetting::None),
            252 => (660, DaylightSavingsTimeSetting::None),
            253 => (600, DaylightSavingsTimeSetting::None),
            254 => (-180, DaylightSavingsTimeSetting::Paraguayan),
            255 => (-300, DaylightSavingsTimeSetting::None),
            256 => (480, DaylightSavingsTimeSetting::None),
            257 => (60, DaylightSavingsTimeSetting::None),
            258 => (-60, DaylightSavingsTimeSetting::European),
            259 => (0, DaylightSavingsTimeSetting::European),
            260 => (180, DaylightSavingsTimeSetting::None),
            261 => (60, DaylightSavingsTimeSetting::None),
            262 => (120, DaylightSavingsTimeSetting::None),
            263 => (480, DaylightSavingsTimeSetting::None),
            264 => (120, DaylightSavingsTimeSetting::None),
            265 => (720, DaylightSavingsTimeSetting::None),
            266 => (420, DaylightSavingsTimeSetting::None),
            267 => (660, DaylightSavingsTimeSetting::None),
            268 => (180, DaylightSavingsTimeSetting::None),
            269 => (360, DaylightSavingsTimeSetting::None),
            270 => (240, DaylightSavingsTimeSetting::None),
            271 => (600, DaylightSavingsTimeSetting::None),
            272 => (540, DaylightSavingsTimeSetting::None),
            273 => (300, DaylightSavingsTimeSetting::None),
            274 => (120, DaylightSavingsTimeSetting::None),
            275 => (-240, DaylightSavingsTimeSetting::None),
            276 => (-240, DaylightSavingsTimeSetting::None),
            277 => (-240, DaylightSavingsTimeSetting::None),
            278 => (780, DaylightSavingsTimeSetting::None),
            279 => (60, DaylightSavingsTimeSetting::European),
            280 => (0, DaylightSavingsTimeSetting::None),
            281 => (180, DaylightSavingsTimeSetting::None),
            282 => (0, DaylightSavingsTimeSetting::None),
            283 => (60, DaylightSavingsTimeSetting::European),
            284 => (240, DaylightSavingsTimeSetting::None),
            285 => (0, DaylightSavingsTimeSetting::None),
            286 => (480, DaylightSavingsTimeSetting::None),
            287 => (60, DaylightSavingsTimeSetting::None),
            288 => (60, DaylightSavingsTimeSetting::None),
            289 => (660, DaylightSavingsTimeSetting::None),
            290 => (180, DaylightSavingsTimeSetting::None),
            291 => (120, DaylightSavingsTimeSetting::None),
            292 => (540, DaylightSavingsTimeSetting::None),
            293 => (120, DaylightSavingsTimeSetting::None),
            294 => (0, DaylightSavingsTimeSetting::European),
            295 => (60, DaylightSavingsTimeSetting::European),
            296 => (330, DaylightSavingsTimeSetting::None),
            297 => (120, DaylightSavingsTimeSetting::None),
            298 => (-180, DaylightSavingsTimeSetting::None),
            299 => (60, DaylightSavingsTimeSetting::None),
            300 => (60, DaylightSavingsTimeSetting::European),
            301 => (180, DaylightSavingsTimeSetting::None),
            302 => (480, DaylightSavingsTimeSetting::None),
            303 => (300, DaylightSavingsTimeSetting::None),
            304 => (180, DaylightSavingsTimeSetting::None),
            305 => (420, DaylightSavingsTimeSetting::None),
            306 => (0, DaylightSavingsTimeSetting::None),
            307 => (780, DaylightSavingsTimeSetting::None),
            308 => (-240, DaylightSavingsTimeSetting::None),
            309 => (60, DaylightSavingsTimeSetting::None),
            310 => (180, DaylightSavingsTimeSetting::None),
            311 => (300, DaylightSavingsTimeSetting::None),
            312 => (720, DaylightSavingsTimeSetting::None),
            313 => (180, DaylightSavingsTimeSetting::None),
            314 => (120, DaylightSavingsTimeSetting::European),
            315 => (240, DaylightSavingsTimeSetting::None),
            316 => (-240, DaylightSavingsTimeSetting::American),
            317 => (0, DaylightSavingsTimeSetting::European),
            318 => (-240, DaylightSavingsTimeSetting::None),
            319 => (-300, DaylightSavingsTimeSetting::None),
            320 => (-180, DaylightSavingsTimeSetting::None),
            321 => (60, DaylightSavingsTimeSetting::European),
            322 => (360, DaylightSavingsTimeSetting::None),
            323 => (120, DaylightSavingsTimeSetting::European),
            324 => (-480, DaylightSavingsTimeSetting::None),
            325 => (0, DaylightSavingsTimeSetting::None),
            326 => (-120, DaylightSavingsTimeSetting::None),
            _ => (0, DaylightSavingsTimeSetting::None),
        }
    }

    fn is_summer_time(timezone: &Timezone, timestamp: &UnixTimestamp) -> bool {
        let (minutes, dst) = Self::offset_minutes_and_dst(timezone);
        match dst {
            DaylightSavingsTimeSetting::None => false,
            DaylightSavingsTimeSetting::American => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_american_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Cuban => ModularTimestamp::from_unix_timestamp(timestamp)
                .is_cuban_summer_time(minutes as i32),
            DaylightSavingsTimeSetting::European => {
                ModularTimestamp::from_unix_timestamp(timestamp).is_european_summer_time()
            }
            DaylightSavingsTimeSetting::Moldovan => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_moldovan_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Lebanese => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_lebanese_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Israeli => ModularTimestamp::from_unix_timestamp(timestamp)
                .is_israeli_summer_time(minutes as i32),
            DaylightSavingsTimeSetting::Palestinian => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_palestinian_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Chilean => {
                ModularTimestamp::from_unix_timestamp(timestamp).is_chilean_summer_time()
            }
            DaylightSavingsTimeSetting::Paraguayan => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_paraguayan_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Australian => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_australian_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::LordHowe => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_lord_howe_summer_time(minutes as i32)
            }
            DaylightSavingsTimeSetting::Kiwi => {
                ModularTimestamp::from_unix_timestamp(timestamp).is_kiwi_summer_time()
            }
            DaylightSavingsTimeSetting::Troll => {
                ModularTimestamp::from_unix_timestamp(timestamp).is_european_summer_time()
            }
            DaylightSavingsTimeSetting::Casey => {
                ModularTimestamp::from_unix_timestamp(timestamp).is_kiwi_summer_time()
            }
            DaylightSavingsTimeSetting::Moroccan => {
                ModularTimestamp::from_unix_timestamp(timestamp)
                    .is_moroccan_summer_time(minutes as i32)
            }
        }
    }

    ///Compares whether two timestamps are equal in all their values. This assumes a 
    ///same timezone.
    pub fn equals(&self, other: &LocalTimestamp) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.date == other.date
            && self.hour == other.hour
            && self.minute == other.minute
            && self.weekday == other.weekday
    }

    ///Compares whether this LocalTimestamp took place after another one, assuming same timezone.
    pub fn is_greater_than(&self, other: &LocalTimestamp) -> bool {
        if self.year > other.year {
            true
        } else if self.year < other.year {
            false
        } else {
            if self.month > other.month {
                true
            } else if self.month < other.month {
                false
            } else {
                if self.date > other.date {
                    true
                } else if self.date < other.date {
                    false
                } else {
                    if self.hour > other.hour {
                        true
                    } else if self.hour < other.hour {
                        false
                    } else {
                        if self.minute > other.minute {
                            true
                        } else if self.minute < other.minute {
                            false
                        } else {
                            self.second > other.second
                        }
                    }
                }
            }
        }
    }

    ///Creates a LocalTimestamp from a UNIX Timestamp in Universal Coordinated Time
    pub fn utc_from_unix_timestamp(input: &UnixTimestamp) -> Self {
        let modular = ModularTimestamp::from_unix_timestamp(input);
        let (month, date) = modular.actual_utc_month_and_date();
        Self {
            year: modular.actual_utc_year(),
            month,
            date,
            hour: modular.get_hour(),
            minute: modular.get_minute(),
            second: modular.get_second(),
            weekday: modular.get_weekday(),
        }
    }

    ///Converts this to a UNIX Timestamp in Universal Coordinated Time.
    pub fn utc_to_unix_timestamp(&self) -> UnixTimestamp {
        let mut year = self.year - 1601;
        if self.month == 1 {
            year -= 1;
        } else if self.month == 2 {
            year -= 1;
        }
        let quadcenturies = year / 400;
        year %= 400;
        let centuries = year / 100;
        year %= 100;
        let quadyears = year / 4;
        let years = year % 4;
        let days = [306, 337, 0, 31, 61, 92, 122, 153, 184, 214, 245, 275]
            [(self.month - 1) as usize]
            + (self.date as u16)
            - 1;
        ModularTimestamp::gen(
            quadcenturies,
            centuries as u8,
            quadyears as u8,
            if days == 365 { 4 } else { years as u8 },
            days % 365,
            self.hour,
            self.minute,
            self.second,
            self.weekday,
        )
        .to_unix_timestamp()
    }

    fn unix_timestamp_from_i64(input: i64) -> UnixTimestamp {
        if input < 0 {
            UnixTimestamp::from_u64(0)
        } else {
            UnixTimestamp::from_u64(input as u64)
        }
    }

    ///Converts a UNIX Timestamp to a LocalTimestamp based on a timezone.
    pub fn from_unix_timestamp(unix_timestamp: &UnixTimestamp, timezone: &Timezone) -> Self {
        let (mut offset, dst) = Self::offset_minutes_and_dst(timezone);
        if Self::is_summer_time(timezone,unix_timestamp) {
            offset += dst.summer_offset_adjustment();
        }
        let offset = offset as i64;
        let unix_timestamp_as_u64 = unix_timestamp.as_u64();
        let unix_timestamp_as_i64 = if unix_timestamp_as_u64 > (i64::MAX as u64) {
            i64::MAX
        } else {
            unix_timestamp_as_u64 as i64
        };
        Self::utc_from_unix_timestamp(&Self::unix_timestamp_from_i64(
            { unix_timestamp_as_i64 } + (offset * 60),
        ))
    }

    ///Returns the Nth of a certain day of the week within a month. For example,
    ///the second tuesday in a month.
    ///Weekday is the day of the week, where 0 is Sunday, et cetera, 6 is Saturday, and
    ///Week placement can be:
    /// - 1, which returns the first of that weekday in the month
    /// - 2, which returns the second of that weekday in the month
    /// - 3, which returns the third of that weekday in the month
    /// - 4, which returns the fourth of that weekday in the month
    /// - 5, which returns the LAST of that weekday in the month
    pub fn from_week_placement(
        year: u64,
        month: u8,
        weekday: u8,
        week_placement: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, ()> {
        if year < 1602 || month == 0 || month > 12 || weekday > 6 {
            Err(())
        } else {
            let l = year - 1;
            let c = 1;
            let m = (if (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0) {
                [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6]
            } else {
                [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5]
            })[(month - 1) as usize];
            let f = ((c + (l / 400) + (l / 4) + l + m - (l / 100)) % 7) as u8; //dow of first day of month
            let first_date_that_is_xday = if f > weekday { 8 } else { 1 } + weekday - f;
            let date = match week_placement {
                1 => first_date_that_is_xday,
                2 => first_date_that_is_xday + 7,
                3 => first_date_that_is_xday + 14,
                4 => first_date_that_is_xday + 21,
                5 => {
                    let x = first_date_that_is_xday + 28;
                    if x > [
                        31,
                        {
                            if (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0) {
                                29
                            } else {
                                28
                            }
                        },
                        31,
                        30,
                        31,
                        30,
                        31,
                        31,
                        30,
                        31,
                        30,
                        31,
                    ][(month - 1) as usize]
                    {
                        x - 7
                    } else {
                        x
                    }
                }
                _ => {
                    return Err(());
                }
            };
            Ok(Self {
                year,
                month,
                date,
                hour,
                minute,
                second,
                weekday,
            })
        }
    }

    ///Converts this to a UNIX Timestamp
    pub fn to_unix_timestamp(&self, timezone: &Timezone) -> UnixTimestamp {
        let mut u: u64 = 0;
        let mut v: u64 = u64::from_be_bytes([0x40,0,0,0,0,0,0,0]);
        for _ in 0..63 {
            let u2 = u | v;
            let x = LocalTimestamp::from_unix_timestamp(&UnixTimestamp::from_u64(u2), timezone);
            if self.is_greater_than(&x) {
                u = u2;
            }
            v >>= 1;
        }
        UnixTimestamp::from_u64(u)
    }
}
