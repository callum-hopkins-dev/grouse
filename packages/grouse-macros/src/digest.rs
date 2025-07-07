use std::{fs::File, io::Read, ops::Deref, path::Path};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Digest([u8; 32]);

impl Digest {
    pub fn from_reader<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: Read,
    {
        let mut sha256 = <sha2::Sha256 as sha2::Digest>::new();
        std::io::copy(reader, &mut sha256)?;

        Ok(Self(
            <sha2::Sha256 as sha2::Digest>::finalize(sha256).into(),
        ))
    }

    #[inline]
    pub fn from_path<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        Self::from_reader(&mut File::open(path)?)
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn to_hex(&self) -> Hex {
        const ALHPHABET: &'static [u8; 16] = b"0123456789abcdef";

        let mut bytes = [0u8; 64];
        let mut index = 0;

        while index < 32 {
            bytes[(index * 2) + 0] = ALHPHABET[(self.0[index] / 16) as usize];
            bytes[(index * 2) + 1] = ALHPHABET[(self.0[index] % 16) as usize];

            index += 1;
        }

        Hex(bytes)
    }
}

impl AsRef<[u8]> for Digest {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for Digest {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Hex([u8; 64]);

impl Hex {
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        unsafe { ::core::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl AsRef<[u8]> for Hex {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Hex {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Hex {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl quote::ToTokens for Hex {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.as_str().to_tokens(tokens);
    }
}
