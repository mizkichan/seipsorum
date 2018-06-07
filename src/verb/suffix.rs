/// 統語動詞接尾辞
#[derive(Debug, Clone)]
pub struct SyntacticalVerbalSuffix {
    /// 連結子音または連結母音
    juncture: Option<char>,

    // NOTE: 名称要検討
    body: String,
}

impl SyntacticalVerbalSuffix {
    pub fn new<T, U>(juncture: T, body: U) -> SyntacticalVerbalSuffix
    where
        T: Into<Option<char>>,
        U: Into<String>,
    {
        SyntacticalVerbalSuffix {
            juncture: juncture.into(),
            body: body.into(),
        }
    }

    pub fn has_juncture_consonant(&self) -> bool {
        match self.juncture {
            Some(juncture) => match juncture {
                'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn has_juncture_vowel(&self) -> bool {
        match self.juncture {
            Some(juncture) => match juncture {
                'a' | 'i' | 'u' | 'e' | 'o' => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn with_juncture(&self) -> String {
        if let Some(juncture) = self.juncture {
            format!("{}{}", juncture, self.body)
        } else {
            self.body.to_string()
        }
    }

    pub fn without_juncture(&self) -> String {
        self.body.clone()
    }
}

/// 派生動詞接尾辞
#[derive(Debug, Clone)]
pub struct DerivationalVerbalSuffix {
    /// 連結子音または連結母音
    juncture: char,

    // NOTE: 名称要検討
    body: String,
}

impl DerivationalVerbalSuffix {
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

    pub fn has_juncture_consonant(&self) -> bool {
        match self.juncture {
            'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
            _ => false,
        }
    }

    pub fn has_juncture_vowel(&self) -> bool {
        match self.juncture {
            'a' | 'i' | 'u' | 'e' | 'o' => true,
            _ => false,
        }
    }

    pub fn with_juncture(&self) -> String {
        format!("{}{}", self.juncture, self.body)
    }

    pub fn without_juncture(&self) -> String {
        self.body.clone()
    }

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

    pub fn is_vowel_stem(&self) -> bool {
        !self.is_consonant_stem()
    }
}
