use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RxTxMap {
    tx_address: u64,
    rx_address: u64,
    page_count: u32,
}

impl RxTxMap {
    pub fn new(tx_address: u64, rx_address: u64, page_count: u32) -> Self {
        Self {
            tx_address,
            rx_address,
            page_count,
        }
    }
}

impl Function for RxTxMap {
    const ID: FunctionId = FunctionId::RxTxMap;
    type ReturnType = ();

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |_| Ok(()))
    }
}

impl TryInto<SmcParams> for RxTxMap {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: self.tx_address,
            x2: self.rx_address,
            x3: self.page_count as u64,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for RxTxMap {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(RxTxMap {
            tx_address: value.x1,
            rx_address: value.x2,
            page_count: value.x3 as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case::zero_values(0, 0, 0)]
    #[case::max_u64_addresses(u64::MAX, u64::MAX, 0)]
    #[case::max_page_count(1, 2, u32::MAX)]
    #[case::typical_values(0x80000000, 0x90000000, 256)]
    fn test_rxtx_map_round_trip(
        #[case] tx_address: u64,
        #[case] rx_address: u64,
        #[case] page_count: u32,
    ) {
        let original_map = RxTxMap {
            tx_address,
            rx_address,
            page_count,
        };

        let params: SmcParams = original_map.clone().try_into().unwrap();
        let new_map: RxTxMap = params.try_into().unwrap();

        assert_eq!(original_map, new_map);
    }
}

// impl FfaRxTxMap {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     fn exec(&self) -> FfaErrorCode {
//         let params: FfaSmcCall = self.into();

//         let result = ffa_smc(params);

//         let err = result.x2 as i64;

//         match FfaFunctionId::try_from(result.id).unwrap() {
//             FfaFunctionId::Success32 => FfaErrorCode::Ok,
//             FfaFunctionId::Error => (err as i64).try_into().unwrap(),
//             _ => panic!("Unknown error"),
//         }
//     }

//     pub fn map(&mut self, tx_addr: u64, rx_addr: u64, page_count: u32) -> FfaErrorCode {
//         self.function_id = FfaFunctionId::RxTxMap.into();
//         self.tx_address = tx_addr;
//         self.rx_address = rx_addr;
//         self.page_count = page_count as u64;

//         self.exec()
//     }

//     pub fn unmap(&mut self, vm_id: u16) -> FfaErrorCode {
//         self.function_id = FfaFunctionId::RxTxUnmap.into();
//         self.tx_address = (vm_id as u64) << 16;
//         self.rx_address = 0;
//         self.page_count = 0;

//         self.exec()
//     }
// }
