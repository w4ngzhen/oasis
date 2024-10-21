pub struct MapGenerate {}

impl MapGenerate {
    pub fn new() -> Self {
        Self {}
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
                let list = self.build_chunk(chunk_size);
                list.iter().for_each(|&x| {
                    map.push(x);
                })
            }
        }
        map
    }

    fn build_chunk(&self, chunk_size: i32) -> Vec<f64> {
        let mut list: Vec<i32> =
            Vec::with_capacity((chunk_size * chunk_size) as usize);
        // 这里我们暂时先固定4个
        let g00 = (-2, 1);
        let g01 = (3, 1);
        let g10 = (-3, -1);
        let g11 = (-1, 2);
        let c00 = (0, 0);
        let c01 = (chunk_size, 0);
        let c10 = (chunk_size, chunk_size);
        let c11 = (0, chunk_size);
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for x in 0..chunk_size {
            for y in 0..chunk_size {
                let v00 = (x - c00.0, y - c00.1);
                let v01 = (x - c01.0, y - c01.1);
                let v10 = (x - c10.0, y - c10.1);
                let v11 = (x - c11.0, y - c11.1);
                let val00 = v00.0 * g00.0 + v00.1 * g00.1;
                let val01 = v01.0 * g01.0 + v01.1 * g01.1;
                let val10 = v10.0 * g10.0 + v10.1 * g10.1;
                let val11 = v11.0 * g11.0 + v11.1 * g11.1;
                let val = val00 + val01 + val10 + val11;
                if val < min {
                    min = val;
                } else if val > max {
                    max = val;
                }
                list.push(val00 + val01 + val10 + val11);
            }
        }
        // 归一化
        if min < 0 {
            list = list.iter().map(|x| *x + min.abs()).collect();
            max += min.abs();
        }
        let result: Vec<f64> =
            list.iter().map(|x| *x as f64 / max as f64).collect();
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
        let generate = MapGenerate::new();
        let result = generate.build(1, 1, 1024);
        println!("{:?}", result);
        let pixels =
            result.iter().map(|x| (*x * 255.) as u8).collect::<Vec<_>>();
        let target = Path::new("example_images/").join(Path::new("map.png"));
        fs::create_dir_all(
            target.clone().parent().expect("No parent directory found."),
        )
        .expect("Failed to create directories.");

        let _ = image::save_buffer(
            target,
            &pixels,
            1 * 1024,
            1 * 1024,
            image::ColorType::L8,
        );
    }
}
