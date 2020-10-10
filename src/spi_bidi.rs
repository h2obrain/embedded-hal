//! Bidirectional Serial Peripheral Interface

use nb;

pub use crate::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

/// Bi-Directional (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select lines
///
/// - Due to how bi-directional SPI works, each time communication is switched between
/// `read` and `send`/`send_finish`, the direction must be set up with a call to `trx_init`
/// to enable SPI again.
/// To disable reading forever or waiting for write to finish `tx_finish` has to be
/// called.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
/// `Word` types to allow operation in both modes.
pub trait BiDirectional<Word> {
    /// An enumeration of SPI errors
    type Error;

    /// Initialize a read or send action
    fn try_trx_init(&mut self, output: bool) -> nb::Result<(), Self::Error>;

    /// Reads the word stored in the shift register
    fn try_read(&mut self, last: bool) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave
    fn try_send(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Finish a read or send action
    fn try_send_finish(&mut self) -> nb::Result<(), Self::Error>;
}
