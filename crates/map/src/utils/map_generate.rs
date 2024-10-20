use noise::core::perlin::perlin_2d;
use noise::permutationtable::PermutationTable;
use noise::utils::{NoiseMap, PlaneMapBuilder};

pub(crate) fn generate_world(seed: u32) -> NoiseMap {
    let hasher = PermutationTable::new(seed);
    let map =
        PlaneMapBuilder::new_fn(|point| octave_perlin(point, &hasher, 1, 2.5))
            .set_is_seamless(true)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    map
}

fn octave_perlin(
    point: [f64; 2],
    hasher: &PermutationTable,
    octaves: usize,
    persistence: f64,
) -> f64 {
    let mut total = 0.;
    let mut frequency = 1.;
    let mut amplitude = 1.;
    let mut max_value = 0.;
    for _ in 0..octaves {
        let [x, y] = point;
        let curr = perlin_2d([x * frequency, y * frequency].into(), hasher)
            * amplitude;
        total += curr;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }
    total / max_value
}

#[cfg(test)]
mod tests {
    use crate::utils::map_generate::generate_world;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test() {
        let map = generate_world(0);
        let (width, height) = map.size();
        let mut pixels: Vec<u8> = Vec::with_capacity(width * height * 8);
        for val in map.iter() {
            let val = (*val * 100.) as i32;
            let mut color: Vec<u8>;
            if val < -80 {
                color = Vec::from([35, 38, 200]);
            } else if val < -60 {
                color = Vec::from([48, 52, 242]);
            } else if val < -40 {
                color = Vec::from([51, 55, 255]);
            } else if val < -20 {
                color = Vec::from([59, 160, 255]);
            } else if val < 0 {
                color = Vec::from([82, 194, 255]);
            } else if val < 20 {
                color = Vec::from([138, 80, 19]);
            } else if val < 40 {
                color = Vec::from([184, 107, 26]);
            } else if val < 60 {
                color = Vec::from([217, 142, 63]);
            } else {
                color = Vec::from([217, 191, 163]);
            }
            pixels.append(&mut color);
        }

        let target = Path::new("example_images/").join(Path::new("map.png"));
        fs::create_dir_all(
            target.clone().parent().expect("No parent directory found."),
        )
        .expect("Failed to create directories.");

        let _ = image::save_buffer(
            target,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        );
    }
}
