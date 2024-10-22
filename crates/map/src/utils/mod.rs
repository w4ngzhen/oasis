pub(crate) mod map_generate;

/// 一个线性插值方法
pub fn linear_interpolation(val1: f64, val2: f64, alpha: f64) -> f64 {
    alpha * val1 + (1.0 - alpha) * val2
}

/// S曲线
///
/// 入参x需要介于0.0 - 1.0之间才有效
///
/// 公式：f(x) = 6x<sup>5</sup> - 15x<sup>4</sup> + 10x<sup>3</sup>（0.0 <= x <= 1.0）
pub fn curve(x: f64) -> f64 {
    if x > 1.0 {
        println!("[warning] x should be in [0.0, 1.0]");
    }
    let x = x.clamp(0.0, 1.0);
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}
