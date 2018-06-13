//! 統語動詞接尾辞

/// 統語動詞接尾辞
#[derive(Debug, Clone)]
pub struct GrammaticalVerbalSuffix {
    /// 連結子音または連結母音
    juncture: Option<char>,

    // NOTE: 名称要検討
    body: String,
}

impl GrammaticalVerbalSuffix {
    /// 接続音と後続部分から統語動詞接尾辞を作成する．
    pub fn new<T, U>(juncture: T, body: U) -> GrammaticalVerbalSuffix
    where
        T: Into<Option<char>>,
        U: Into<String>,
    {
        GrammaticalVerbalSuffix {
            juncture: juncture.into(),
            body: body.into(),
        }
    }

    /// 統語動詞接尾辞が接続子音を持っている場合に `true` を返す．
    pub fn has_juncture_consonant(&self) -> bool {
        match self.juncture {
            Some(juncture) => match juncture {
                'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
                _ => false,
            },
            None => false,
        }
    }

    /// 統語動詞接尾辞が接続母音を持っている場合に `true` を返す．
    pub fn has_juncture_vowel(&self) -> bool {
        match self.juncture {
            Some(juncture) => match juncture {
                'a' | 'i' | 'u' | 'e' | 'o' => true,
                _ => false,
            },
            None => false,
        }
    }

    /// 接続音を含んだ形の統語動詞接尾辞を `String` で返す．
    pub fn with_juncture(&self) -> String {
        if let Some(juncture) = self.juncture {
            format!("{}{}", juncture, self.body)
        } else {
            self.body.to_string()
        }
    }

    /// 接続音を含まない形の統語動詞接尾辞を `String` で返す．
    pub fn without_juncture(&self) -> String {
        self.body.clone()
    }
}