//! TeX category codes and the stateful lookup table.
//!
//! Each character in a TeX source file is assigned a **catcode** (0–17) that
//! determines how the tokenizer treats it. [`LtxCatCodeState`] is a 256-byte
//! lookup table that maps `char → LtxCatCode` in O(1).

/// TeX category code for a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LtxCatCode {
    /// Escape character. \
    Escape = 0,
    /// Begin group character. \{
    GroupStart = 1,
    /// End group character. \}
    GroupEnd = 2,
    /// Math shift character. \$
    MathShift = 5,
    /// Alignment tab character. \&
    AlignmentTab = 6,
    /// End of line character. \n
    EndOfLine = 7,
    /// Parameter character. \#
    Parameter = 8,
    /// Superscript character. \^
    Superscript = 9,
    /// Subscript character. \_
    Subscript = 10,
    /// Ignored character. \null
    Ignored = 11,
    /// Space character. \space
    WhiteSpace = 12,
    /// Letter character. \letter
    Letter = 13,
    /// Other character. \other
    Other = 14,
    /// Active character. \active
    Active = 15,
    /// Comment character. \comment
    Comment = 16,
    /// Invalid character. \invalid
    Invalid = 17,
}

impl LtxCatCode {
    /// Converts a raw byte value to a `CatCode`.
    #[must_use]
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Escape),
            1 => Some(Self::GroupStart),
            2 => Some(Self::GroupEnd),
            5 => Some(Self::MathShift),
            6 => Some(Self::AlignmentTab),
            7 => Some(Self::EndOfLine),
            8 => Some(Self::Parameter),
            9 => Some(Self::Superscript),
            10 => Some(Self::Subscript),
            11 => Some(Self::Ignored),
            12 => Some(Self::WhiteSpace),
            13 => Some(Self::Letter),
            14 => Some(Self::Other),
            15 => Some(Self::Active),
            16 => Some(Self::Comment),
            17 => Some(Self::Invalid),
            _ => None,
        }
    }

    /// Returns the raw byte value of the `CatCode`.
    #[inline]
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
/// A stateful `CatCode` lookup table using a fixed-size array for optimal performance.
pub struct LtxCatCodeState {
    map: [u8; 256], // 256 bytes, cache-friendly
}

impl LtxCatCodeState {
    /// Create a default TeX catcode table
    ///
    /// # Returns
    ///
    /// The default TeX catcode table
    #[must_use]
    #[inline]
    pub fn default_tex() -> Self {
        let mut map = [LtxCatCode::Other.as_u8(); 256]; // Everything = Other

        // `$` is mapped to `MathShift` — the InlineMathStart/InlineMathEnd
        // distinction is lexer *state* (odd/even `$` count), not a catcode property.
        map[b'\\' as usize] = LtxCatCode::Escape.as_u8();
        map[b'{' as usize] = LtxCatCode::GroupStart.as_u8();
        map[b'}' as usize] = LtxCatCode::GroupEnd.as_u8();
        map[b'$' as usize] = LtxCatCode::MathShift.as_u8();
        map[b'&' as usize] = LtxCatCode::AlignmentTab.as_u8();
        map[b'\n' as usize] = LtxCatCode::EndOfLine.as_u8();
        map[b'\r' as usize] = LtxCatCode::EndOfLine.as_u8();
        map[b'\0' as usize] = LtxCatCode::Ignored.as_u8();
        map[b'#' as usize] = LtxCatCode::Parameter.as_u8();
        map[b'^' as usize] = LtxCatCode::Superscript.as_u8();
        map[b'_' as usize] = LtxCatCode::Subscript.as_u8();
        map[b' ' as usize] = LtxCatCode::WhiteSpace.as_u8();
        map[b'~' as usize] = LtxCatCode::Active.as_u8();
        map[b'%' as usize] = LtxCatCode::Comment.as_u8();

        // Letters (A-Z, a-z)
        for c in (b'A'..=b'Z').chain(b'a'..=b'z') {
            map[c as usize] = LtxCatCode::Letter.as_u8();
        }

        Self { map }
    }

    /// Get catcode for character
    #[inline]
    #[must_use]
    pub const fn get(&self, c: char) -> LtxCatCode {
        let val = c as u32;
        if val >= 256 {
            return LtxCatCode::Other;
        }
        match LtxCatCode::from_u8(self.map[val as usize]) {
            Some(cat) => cat,
            None => LtxCatCode::Other,
        }
    }

    /// Is this character a letter?
    #[inline]
    #[must_use]
    pub fn is_letter(&self, c: char) -> bool {
        let cat = self.get(c);

        cat == LtxCatCode::Letter || c.is_alphabetic()
    }
    /// Set the catcode for a character
    #[inline]
    pub const fn set(&mut self, c: char, cat: LtxCatCode) {
        let byte = c as u32;
        if byte < 256 {
            self.map[byte as usize] = cat.as_u8();
        }
        // debug_assert is not available in const fn, but callers should
        // ensure `c` is Latin-1 or the assignment is silently ignored
    }

    /// Reset everything to "Other" (useful for verbatim mode)
    #[inline]
    pub fn reset_to_other(&mut self) {
        self.map.fill(LtxCatCode::Other.as_u8());
    }
}

impl Default for LtxCatCodeState {
    fn default() -> Self {
        Self::default_tex()
    }
}
