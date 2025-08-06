use std::mem::MaybeUninit;

use rand::{Rng, RngCore};


/// 终端电力状态数据
#[derive(Debug, Clone, Copy)]
pub struct SendingData {
    /// 编号
    pub id: u64,
    /// 电压
    pub voltage: f64,
    /// 电流
    pub current: f64,
    /// 功率
    pub power: f64,
    /// 功率因数
    pub power_factor: f64,
    /// 频率
    pub frequency: f64,
    /// 累计有功功率
    pub total_active_power: f64,
    /// 累计无功功率
    pub total_reactive_power: f64,
}

/// SendingData 数据长度
/// (反正都固定的了 没必要重复获取
pub const DATA_LEN: usize = std::mem::size_of::<SendingData>();

impl From<&[u8]> for SendingData {
    fn from(value: &[u8]) -> Self {
        // I love MaybeUninit
        let mut data = MaybeUninit::<SendingData>::uninit();
        unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), data.as_mut_ptr() as *mut _, DATA_LEN) };
        unsafe { data.assume_init() }
    }
}


impl From<&SendingData> for Vec<u8> {
    fn from(value: &SendingData) -> Self {
        let mut data = Vec::with_capacity(DATA_LEN);
        // 取地址, copy byte
        // 我估计你看到这里会被吓到（确信
        // 但这不就是最简单的 serde/deserialize 吗 (乐)
        unsafe { std::ptr::copy_nonoverlapping(value as *const SendingData, data.as_mut_ptr() as *mut _, DATA_LEN) };
        data
    }
}

impl SendingData {
    pub fn new_rand(rander: &mut rand::rngs::ThreadRng) -> Self {
        let id: u64 = (rander.sample(rand::distr::Uniform::new(1, 1_0000_0000).unwrap()) as f32).sqrt() as u64;
        let voltage: f64 = rander.sample(rand::distr::Uniform::new(220.0, 1000.0).unwrap());
        let current: f64 = rander.sample(rand::distr::Uniform::new(0.5, 1000.0).unwrap());
        let power: f64 = rander.sample(rand::distr::Uniform::new(0.0, 1000_000.0).unwrap());
        let power_factor: f64 = rander.sample(rand::distr::Uniform::new(0.5, 1.0).unwrap());
        let frequency: f64 = rander.sample(rand::distr::Uniform::new(50.0, 100.0).unwrap());
        let total_active_power: f64 = rander.sample(rand::distr::Uniform::new(0.0, 1000_000.0).unwrap());
        let total_reactive_power: f64 = rander.sample(rand::distr::Uniform::new(0.0, 1000_000.0).unwrap());
        Self {
            id,
            voltage,
            current,
            power,
            power_factor,
            frequency,
            total_active_power,
            total_reactive_power,
        }
    }
}