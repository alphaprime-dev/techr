use crate::overlap::ema;

pub fn efi(closes: &[f64], volumes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut efi = vec![None; len];

    if len != volumes.len() || len < 2 {
        return efi;
    }

    let force: Vec<f64> = closes
        .windows(2)
        .zip(volumes.iter().skip(1))
        .map(|(window, &volume)| (window[1] - window[0]) * volume)
        .collect();

    if period == 1 {
        efi.iter_mut()
            .skip(1)
            .zip(force.iter())
            .for_each(|(efi_val, &force_val)| {
                *efi_val = Some(force_val);
            });
    } else {
        let ema_result = ema(&force, period);
        efi.iter_mut()
            .skip(1)
            .zip(ema_result.into_iter())
            .for_each(|(efi_val, ema_val)| {
                *efi_val = ema_val;
            });
    }

    efi
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_efi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");
            let result = efi(&close, &volume, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/efi_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 4),
                round_vec(expected, 4),
                "EFI test failed for symbol {}.",
                symbol
            );
        }
    }
}
