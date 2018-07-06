//! 文法動詞接尾辞

/// 文法動詞接尾辞
trait GrammaticalVerbalSuffix {
    fn get_juncture(&self) -> Option<char>;
    fn get_body(&self) -> &str;

    /// 文法動詞接尾辞が接続子音を持っている場合に `true` を返す．
    fn has_juncture_consonant(&self) -> bool {
        match self.get_juncture() {
            Some(juncture) => match juncture {
                'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
                _ => false,
            },
            None => false,
        }
    }

    /// 文法動詞接尾辞が接続母音を持っている場合に `true` を返す．
    fn has_juncture_vowel(&self) -> bool {
        match self.get_juncture() {
            Some(juncture) => match juncture {
                'a' | 'i' | 'u' | 'e' | 'o' => true,
                _ => false,
            },
            None => false,
        }
    }

    /// 接続音を含んだ形の文法動詞接尾辞を `String` で返す．
    fn with_juncture(&self) -> String {
        let body = self.get_body();
        if let Some(juncture) = self.get_juncture() {
            format!("{}{}", juncture, body)
        } else {
            body.to_owned()
        }
    }

    /// 接続音を含まない形の文法動詞接尾辞を `String` で返す．
    fn without_juncture(&self) -> String {
        self.get_body().to_owned()
    }
}

/// 動作動詞終止・連体形形成の文法派生接尾辞
pub enum FiniteSuffixForActionVerb {
    /// 非完了態
    NonPerfective,

    /// 完了態
    Perfective,

    /// 前望態
    Prospective,

    /// 前望態否定
    NegativeProspective,
}

impl GrammaticalVerbalSuffix for FiniteSuffixForActionVerb {
    fn get_juncture(&self) -> Option<char> {
        use self::FiniteSuffixForActionVerb::*;

        Some(match self {
            NonPerfective => 'r',
            Perfective => 'i',
            Prospective => 'y',
            NegativeProspective => 'u',
        })
    }

    fn get_body(&self) -> &str {
        use self::FiniteSuffixForActionVerb::*;

        match self {
            NonPerfective => "u",
            Perfective => "ta",
            Prospective => "oo",
            NegativeProspective => "mai",
        }
    }
}

/// 動作動詞連用形形成の文法接尾辞
pub enum ConverbSuffixForActionVerb {
    /// 順接
    Copulative,

    /// 完了
    Perfective,

    /// 譲歩
    Concessive,

    /// 却下条件
    RejectedConditional,

    /// 開放条件
    OpenConditional,

    /// 仮定条件
    ProvisionalConditional,

    /// 完了条件
    PerfectiveCondition,

    /// 否定
    Negative,

    /// 同時
    Simultaneous,

    /// 目的
    Purposive,

    /// 同時進行
    SimultaneousProcessive,

    /// 前望譲歩
    ProspectiveConcessive,
}

impl GrammaticalVerbalSuffix for ConverbSuffixForActionVerb {
    fn get_juncture(&self) -> Option<char> {
        use self::ConverbSuffixForActionVerb::*;

        Some(match self {
            Copulative => 'i',
            Perfective => 'i',
            Concessive => 'i',
            RejectedConditional => 'i',
            OpenConditional => 'r',
            ProvisionalConditional => 'r',
            PerfectiveCondition => 'i',
            Negative => 'a',
            Simultaneous => 'i',
            Purposive => 'i',
            SimultaneousProcessive => 'i',
            ProspectiveConcessive => 'y',
        })
    }

    fn get_body(&self) -> &str {
        use self::ConverbSuffixForActionVerb::*;

        match self {
            Copulative => "",
            Perfective => "te",
            Concessive => "temo",
            RejectedConditional => "tewa",
            OpenConditional => "uto",
            ProvisionalConditional => "eba",
            PerfectiveCondition => "tara",
            Negative => "zu", // NOTE "zuni" もある
            Simultaneous => "nagara",
            Purposive => "ni",
            SimultaneousProcessive => "tutu",
            ProspectiveConcessive => "ooto",
        }
    }
}

/// 形状動詞終止・連体形形成の文法派生接尾辞
pub enum FiniteSuffixForQualicativeVerb {
    /// 非完了態
    NonPerfective,

    /// 完了態
    Perfective,

    /// 前望態
    Prospective,
}

impl GrammaticalVerbalSuffix for FiniteSuffixForQualicativeVerb {
    fn get_juncture(&self) -> Option<char> {
        None
    }

    fn get_body(&self) -> &str {
        use self::FiniteSuffixForQualicativeVerb::*;

        match self {
            NonPerfective => "i",
            Perfective => "katta",
            Prospective => "karoo",
        }
    }
}
