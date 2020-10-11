//! Blocking Bidirectional SPI API

/// Blocking read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Reads `words` from the responder
    fn try_read<'w>(&mut self, words: &'w mut [W]) -> Result<&'w mut [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the responder
    fn try_write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking iter-read
pub trait ReadIter<W> {
    /// Error type
    type Error;

    /// Reads `words` from the responder
    fn try_read_iter<'l, T>(&mut self, words: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = &'l mut W>,
        W: 'l,
    ;
}

/// Blocking iter-write
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `words` to the responder
    fn try_write_iter<T: IntoIterator<Item = W>>(&mut self, words: T) -> Result<(), Self::Error>;
}

/// Blocking read
pub mod read {
    /// Default implementation of `blocking::spi_bidi::Read<W>` for implementers of `spi_bidi<W>`
    pub trait Default<W>: crate::spi_bidi::BiDirectional<W> {}

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

    impl<W, S> crate::blocking::spi_bidi::ReadIter<W> for S
    where S: Default<W>,
    {
        type Error = S::Error;

        fn try_read_iter<'l, T>(&mut self, words: T) -> Result<(), Self::Error>
        where
            T: IntoIterator<Item = &'l mut W>,
            W: 'l,
         {
            nb::block!(self.try_trx_init(false))?;
            let mut ws = words.into_iter();
            let mut wn = ws.next();
            while wn.is_some() {
                let w = wn;
                wn = ws.next();
                *(w.unwrap()) = nb::block!(self.try_read(!wn.is_some()))?;
            }
            Ok(())
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi_bidi<W>`
    pub trait Default<W>: crate::spi_bidi::BiDirectional<W> {}

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

    impl<W, S> crate::blocking::spi_bidi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write_iter<T: IntoIterator<Item = W>>(&mut self, words: T) -> Result<(), Self::Error> {
            nb::block!(self.try_trx_init(true))?;

            for word in words {
                nb::block!(self.try_send(word.clone()))?;
            }

            nb::block!(self.try_send_finish())?;

            Ok(())
        }
    }
}
