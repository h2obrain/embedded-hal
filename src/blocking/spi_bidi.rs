//! Blocking Bidirectional SPI API

/// Blocking read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    //fn read(&mut self, words: &[W]) -> Result<&[W], Self::Error>;
    fn try_read<'w>(&mut self, words: &'w mut [W]) -> Result<&'w mut [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking read
pub mod read {
    /// Default implementation of `blocking::spi::BidiRead<W>` for implementers of `spi::BiDi<W>`
    //pub trait Default<W>: ::spi::BiDirectional<W> {}
    pub trait Default<W>: crate::spi_bidi::BiDirectional<W> {}

    //impl<W, S> blocking::spi::BidiRead<W> for S
    impl<W, S> crate::blocking::spi_bidi::Read<W> for S
    where
        S: Default<W>,
    {
        type Error = S::Error;

        fn try_read<'l>(&mut self, words: &'l mut [W]) -> Result<&'l mut [W], S::Error> {
            nb::block!(self.try_trx_init(false))?;

            let last = words.iter().last().unwrap() as *const W;
            for word in words.iter_mut() {
                *word = nb::block!(self.try_read(word as *const W == last))?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`
    //pub trait Default<W>: ::spi::BiDirectional<W> {}
    pub trait Default<W>: crate::spi_bidi::BiDirectional<W> {}

    //impl<W, S> blocking::spi::BidiWrite<W> for S
    impl<W, S> crate::blocking::spi_bidi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write(&mut self, words: &[W]) -> Result<(), S::Error> {
            nb::block!(self.try_trx_init(true))?;

            for word in words {
                nb::block!(self.try_send(word.clone()))?;
            }

            nb::block!(self.try_send_finish())?;

            Ok(())
        }
    }
}
