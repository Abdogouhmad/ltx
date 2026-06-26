//! Categories code of latex

/// Categories code of latex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CatCode {
    /// Escape character. \
    Escape = 0, // \
    /// Begin group character. \{
    BeginGroup = 1, // {
    /// End group character. \}
    EndGroup = 2, // }
    /// Math shift character. \$
    MathShift = 3, // $
    /// Alignment tab character. \&
    AlignmentTab = 4,
    /// End of line character. \n
    EndOfLine = 5,
    /// Parameter character. \#
    Parameter = 6,
    /// Superscript character. \^
    Superscript = 7,
    /// Subscript character. \_
    Subscript = 8,
    /// Ignored character. \null
    Ignored = 9,
    /// Space character. \space
    Space = 10,
    /// Letter character. \letter
    Letter = 11,
    /// Other character. \other
    Other = 12,
    /// Active character. \active
    Active = 13,
    /// Comment character. \comment
    Comment = 14,
    /// Invalid character. \invalid
    Invalid = 15,
}

impl CatCode {
    /// Converts a raw byte value to a `CatCode`.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Escape),
            1 => Some(Self::BeginGroup),
            2 => Some(Self::EndGroup),
            3 => Some(Self::MathShift),
            4 => Some(Self::AlignmentTab),
            5 => Some(Self::EndOfLine),
            6 => Some(Self::Parameter),
            7 => Some(Self::Superscript),
            8 => Some(Self::Subscript),
            9 => Some(Self::Ignored),
            10 => Some(Self::Space),
            11 => Some(Self::Letter),
            12 => Some(Self::Other),
            13 => Some(Self::Active),
            14 => Some(Self::Comment),
            15 => Some(Self::Invalid),
            _ => None,
        }
    }

    /// Returns the raw byte value of the `CatCode`.
    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
/// A stateful `CatCode` lookup table using a fixed-size array for optimal performance.
pub struct CatCodeState {
    map: [u8; 256], // 256 bytes, cache-friendly
}

impl CatCodeState {
    /// Create a default TeX catcode table
    ///
    /// # Returns
    ///
    /// The default TeX catcode table
    #[must_use]
    #[inline]
    pub fn default_tex() -> Self {
        let mut map = [12u8; 256]; // Everything = Other (12)

        // Map characters → catcode values
        map[b'\\' as usize] = CatCode::Escape.as_u8(); // 0
        map[b'{' as usize] = CatCode::BeginGroup.as_u8(); // 1
        map[b'}' as usize] = CatCode::EndGroup.as_u8(); // 2
        map[b'$' as usize] = CatCode::MathShift.as_u8(); // 3
        map[b'&' as usize] = CatCode::AlignmentTab.as_u8(); // 4
        map[b'\n' as usize] = CatCode::EndOfLine.as_u8(); // 5
        map[b'#' as usize] = CatCode::Parameter.as_u8(); // 6
        map[b'^' as usize] = CatCode::Superscript.as_u8(); // 7
        map[b'_' as usize] = CatCode::Subscript.as_u8(); // 8
        map[b' ' as usize] = CatCode::Space.as_u8(); // 10
        map[b'~' as usize] = CatCode::Active.as_u8(); // 13
        map[b'%' as usize] = CatCode::Comment.as_u8(); // 14

        // Letters (A-Z, a-z)
        for c in b'A'..=b'Z' {
            map[c as usize] = CatCode::Letter.as_u8();
        }
        for c in b'a'..=b'z' {
            map[c as usize] = CatCode::Letter.as_u8();
        }

        CatCodeState { map }
    }

    /// Get catcode for character
    ///
    /// # Arguments
    ///
    /// * `c` - The character to get the catcode for
    ///
    /// # Returns
    ///
    /// The catcode for the character
    #[inline]
    #[must_use]
    pub fn get(&self, c: char) -> CatCode {
        if c as u32 >= 256 {
            return CatCode::Other;
        }
        CatCode::from_u8(self.map[c as usize]).unwrap_or(CatCode::Other)
    }

    /// Is this character a letter?
    #[inline]
    #[must_use]
    pub fn is_letter(&self, c: char) -> bool {
        self.get(c) == CatCode::Letter
    }

    /// Set the catcode for a character
    #[inline]
    pub fn set(&mut self, c: char, cat: CatCode) {
        let byte = c as u32;
        if byte < 256 {
            self.map[byte as usize] = cat.as_u8();
        }
    }

    /// Reset everything to "Other" (useful for verbatim mode)
    #[inline]
    pub fn reset_to_other(&mut self) {
        for i in 0..256 {
            self.map[i] = CatCode::Other.as_u8();
        }
    }
}

impl Default for CatCodeState {
    fn default() -> Self {
        Self::default_tex()
    }
}
