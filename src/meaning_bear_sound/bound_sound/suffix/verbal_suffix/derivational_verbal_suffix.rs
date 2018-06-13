//! 派生動詞接尾辞

/// 派生動詞接尾辞
#[derive(Debug, Clone)]
pub struct DerivationalVerbalSuffix {
    /// 連結子音または連結母音
    juncture: char,

    // NOTE: 名称要検討
    body: String,
}

impl DerivationalVerbalSuffix {
    /// 接続音と後続部分から派生動詞接尾辞を作成する．
    pub fn new<T, U>(juncture: T, body: U) -> DerivationalVerbalSuffix
    where
        T: Into<char>,
        U: Into<String>,
    {
        DerivationalVerbalSuffix {
            juncture: juncture.into(),
            body: body.into(),
        }
    }

    /// 派生動詞接尾辞が接続子音を持っている場合に `true` を返す．
    pub fn has_juncture_consonant(&self) -> bool {
        match self.juncture {
            'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
            _ => false,
        }
    }

    /// 派生動詞接尾辞が接続母音を持っている場合に `true` を返す．
    pub fn has_juncture_vowel(&self) -> bool {
        match self.juncture {
            'a' | 'i' | 'u' | 'e' | 'o' => true,
            _ => false,
        }
    }

    /// 接続音を含んだ形の派生動詞接尾辞を `String` で返す．
    pub fn with_juncture(&self) -> String {
        format!("{}{}", self.juncture, self.body)
    }

    /// 接続音を含まない形の派生動詞接尾辞を `String` で返す．
    pub fn without_juncture(&self) -> String {
        self.body.clone()
    }

    /// 派生動詞接尾辞が子音幹接尾辞である場合に `true` を返す．
    // NOTE: stem という表現は微妙では？
    pub fn is_consonant_stem(&self) -> bool {
        let last_characher = self.body.chars().last().unwrap();
        assert!(match last_characher {
            'c' | 'f' | 'j' | 'l' | 'q' | 'v' | 'x' => false,
            other => other.is_ascii_lowercase(),
        });

        match last_characher {
            'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
            'a' | 'i' | 'u' | 'e' | 'o' => false,
            _ => unreachable!(),
        }
    }

    /// 派生動詞接尾辞が母音幹接尾辞である場合に `true` を返す．
    pub fn is_vowel_stem(&self) -> bool {
        !self.is_consonant_stem()
    }
}
