//! Dinero sintético expresado en unidades menores enteras.

/// Monedas suficientes para los ejemplos del curso; no son un catálogo ISO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Currency {
    /// Peso mexicano sintético.
    Mxn,
    /// Dólar estadounidense sintético.
    Usd,
}

/// Cantidad local en unidad menor y moneda explícita.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Money {
    minor_units: i64,
    currency: Currency,
}

impl Money {
    /// Crea una cantidad sintética sin hacer conversiones.
    #[must_use]
    pub const fn new(minor_units: i64, currency: Currency) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    /// Devuelve las unidades menores exactas.
    #[must_use]
    pub const fn minor_units(self) -> i64 {
        self.minor_units
    }

    /// Devuelve la moneda asociada a la cantidad.
    #[must_use]
    pub const fn currency(self) -> Currency {
        self.currency
    }

    /// Suma cantidades de la misma moneda.
    pub fn checked_add(self, other: Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        let minor_units = self
            .minor_units
            .checked_add(other.minor_units)
            .ok_or(MoneyError::Overflow)?;
        Ok(Self::new(minor_units, self.currency))
    }

    /// Divide con la regla de redondeo que el llamador declara.
    pub fn split(self, parts: i64, rounding: Rounding) -> Result<Self, MoneyError> {
        if parts <= 0 {
            return Err(MoneyError::InvalidParts);
        }
        let quotient = self.minor_units / parts;
        let remainder = self.minor_units % parts;
        let minor_units = match rounding {
            Rounding::RejectRemainder if remainder != 0 => return Err(MoneyError::Remainder),
            Rounding::RejectRemainder | Rounding::TowardZero => quotient,
            Rounding::AwayFromZero if remainder == 0 => quotient,
            Rounding::AwayFromZero if self.minor_units.is_positive() => quotient + 1,
            Rounding::AwayFromZero => quotient - 1,
        };
        Ok(Self::new(minor_units, self.currency))
    }
}

/// Regla explícita para una división con residuo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rounding {
    RejectRemainder,
    TowardZero,
    AwayFromZero,
}

/// Fallas del contrato aritmético sintético.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoneyError {
    CurrencyMismatch,
    Overflow,
    InvalidParts,
    Remainder,
}

#[cfg(test)]
mod tests {
    use super::{Currency, Money, MoneyError, Rounding};

    #[test]
    fn no_suma_monedas_distintas() {
        assert_eq!(
            Money::new(100, Currency::Mxn).checked_add(Money::new(100, Currency::Usd)),
            Err(MoneyError::CurrencyMismatch)
        );
    }

    #[test]
    fn el_redondeo_debe_ser_explicito() {
        let amount = Money::new(10, Currency::Mxn);
        assert_eq!(
            amount.split(3, Rounding::RejectRemainder),
            Err(MoneyError::Remainder)
        );
        assert_eq!(
            amount
                .split(3, Rounding::AwayFromZero)
                .expect("división permitida")
                .minor_units(),
            4
        );
    }
}
