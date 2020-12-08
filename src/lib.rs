#![no_std]

const ADC_CFGR_RES_BITOFFSET_POS: u32 = 3;

const TEMPSENSOR_CAL_VREFANALOG: u16 = 3000;

const TEMPSENSOR_CAL1_TEMP: u32 = 30;
const TEMPSENSOR_CAL2_TEMP: u32 = 110;

pub enum ADCResolution {
    Bits12 = 0x0,
    Bits10 = 0x1 << 3,
    Bits8 = 0x2 << 3,
    Bits6 = (0x2 << 3) | (0x1 << 3),
}

impl ADCResolution {
    fn adc_convert_data_resolution(data: u16, from: ADCResolution, to: ADCResolution) -> u16 {
        (data << (from as u32 >> (ADC_CFGR_RES_BITOFFSET_POS - 1u32)))
            >> (to as u32 >> (ADC_CFGR_RES_BITOFFSET_POS - 1u32))
    }
}

pub fn adc_calc_temperature(
    vref_analog_voltage: u16,
    adc_data: u16,
    adc_resolution: ADCResolution,
    tempsensor_cal1_val: u16,
    tempsensor_cal2_val: u16,
) -> i32 {
    ((((((ADCResolution::adc_convert_data_resolution(
        adc_data,
        adc_resolution,
        ADCResolution::Bits12,
    ) as u32
        * vref_analog_voltage as u32)
        / TEMPSENSOR_CAL_VREFANALOG as u32) as i32)
        - tempsensor_cal1_val as i32)
        * ((TEMPSENSOR_CAL2_TEMP - TEMPSENSOR_CAL1_TEMP) as i32))
        / ((tempsensor_cal2_val as i32) - (tempsensor_cal1_val as i32)))
        + TEMPSENSOR_CAL1_TEMP as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let val1 = 1034;
        let val2 = 1373;

        let x = adc_calc_temperature(2450, 1259, ADCResolution::Bits12, val1, val2);
        assert_eq!(29, x);
    }

    #[test]
    fn resolution_convert_id() {
        let x = ADCResolution::adc_convert_data_resolution(
            1245,
            ADCResolution::Bits12,
            ADCResolution::Bits12,
        );
        assert_eq!(1245, x);
    }
}
