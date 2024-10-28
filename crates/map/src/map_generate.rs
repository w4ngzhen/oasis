use crate::hasher::Hasher;
use crate::utils::linear_interpolation;
use noise::permutationtable::{NoiseHasher, PermutationTable};

pub struct MapGenerate {
    hasher: Hasher,
}

impl MapGenerate {
    pub fn new(seed: i32) -> Self {
        Self { hasher: Hasher::new(seed) }
    }

    pub fn build(
        &self,
        chunk_w: i32,
        chunk_h: i32,
        chunk_size: i32,
    ) -> Vec<f64> {
        let mut map: Vec<f64> = Vec::with_capacity(
            (chunk_w * chunk_h * chunk_size * chunk_size) as usize,
        );
        for chunk_x in 0..chunk_w {
            for chunk_y in 0..chunk_h {
                let list = self.build_chunk(chunk_x, chunk_y, chunk_size);
                list.iter().for_each(|&x| {
                    map.push(x);
                })
            }
        }
        map
    }

    fn build_chunk(
        &self,
        chunk_x: i32,
        chunk_y: i32,
        chunk_size: i32,
    ) -> Vec<f64> {
        let c_lt = (chunk_x + 0, chunk_y + 0);
        let c_rt = (chunk_x + chunk_size, chunk_y + 0);
        let c_lb = (chunk_x + 0, chunk_y + chunk_size);
        let c_rb = (chunk_x + chunk_size, chunk_y + chunk_size);

        // 四个角的伪随机梯度
        let g_lt = self.hasher.hash(&(c_lt.0 as f64, c_lt.1 as f64));
        let g_rt = self.hasher.hash(&(c_rt.0 as f64, c_rt.1 as f64));
        let g_lb = self.hasher.hash(&(c_lb.0 as f64, c_lb.1 as f64));
        let g_rb = self.hasher.hash(&(c_rb.0 as f64, c_rb.1 as f64));
        // println!(
        //     "{:?}, {:?}, {:?}, {:?}",
        //     (g_lt.sin(), g_lt.cos()),
        //     (g_rt.sin(), g_rt.cos()),
        //     (g_lb.sin(), g_lb.cos()),
        //     (g_rb.sin(), g_rb.cos())
        // );
        let mut result: Vec<f64> =
            Vec::with_capacity((chunk_size * chunk_size) as usize);
        for x in 0..chunk_size {
            for y in 0..chunk_size {
                let vec_lt = (x - c_lt.0, y - c_lt.1);
                let vec_rt = (x - c_rt.0, y - c_rt.1);
                let vec_lb = (x - c_lb.0, y - c_lb.1);
                let vec_rb = (x - c_rb.0, y - c_rb.1);
                let scalar_lt =
                    vec_lt.0 as f64 * g_lt.cos() + vec_lt.1 as f64 * g_lt.sin();
                let scalar_rt =
                    vec_rt.0 as f64 * g_rt.cos() + vec_rt.1 as f64 * g_rt.sin();
                let scalar_lb =
                    vec_lb.0 as f64 * g_lb.cos() + vec_lb.1 as f64 * g_lb.sin();
                let scalar_rb =
                    vec_rb.0 as f64 * g_rb.cos() + vec_rb.1 as f64 * g_rb.sin();

                // 1. 将top的left和right线性插值到一起，得到结果top线性插值结果，tli(top_linear_interpolation)
                // 2. 将bottom的left和right线性插值到一起，得到结果bottom线性插值结果
                // 3. 将将tli和bli再次进行线性插值
                let top_val = linear_interpolation(scalar_lt, scalar_lb, 0.5);
                let bottom_val =
                    linear_interpolation(scalar_rt, scalar_rb, 0.5);
                let val = linear_interpolation(top_val, bottom_val, 0.5);
                result.push(val);
            }
        }
        println!("{:?}", result);
        result
    }
}
#[cfg(test)]
mod tests {
    use crate::map_generate::MapGenerate;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test() {
        let generate = MapGenerate::new(123);
        let chunk_w = 16;
        let chunk_h = 16;
        let chunk_size = 64;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut result = generate
            .build(chunk_w, chunk_h, chunk_size)
            .iter()
            .map(|x| {
                let val = (*x * 1000.) as i32;
                if val > max {
                    max = val;
                } else if val < min {
                    min = val;
                }
                val
            })
            .collect::<Vec<_>>();
        if min < 0 {
            result = result.iter().map(|x| x + min.abs()).collect();
            max += min.abs();
        }

        println!("{:?}", result);
        let pixels = result
            .iter()
            .map(|x| ((*x as f64 / max as f64) * 255.) as u8)
            .collect::<Vec<_>>();
        let target = Path::new("example_images/").join(Path::new("map.png"));
        fs::create_dir_all(
            target.clone().parent().expect("No parent directory found."),
        )
        .expect("Failed to create directories.");

        let _ = image::save_buffer(
            target,
            &pixels,
            (chunk_w * chunk_size) as u32,
            (chunk_h * chunk_size) as u32,
            image::ColorType::L8,
        );
    }
}
