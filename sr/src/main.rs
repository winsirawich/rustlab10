// Q1.1)
// use rand::{Rng, SeedableRng};
// use rand::rngs::SmallRng;

// #[derive(Debug)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }

// #[derive(Debug)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }

// fn gen_obj_layer_list(seed: u64, n: usize) -> Vec<Layer> {
//     let mut rng = SmallRng::seed_from_u64(seed);

//     let mut layers = Vec::new();
//     for i in 0..n {
//         let name = format!("Layer {}", i);
//         let color = format!("#{:02X}{:02X}{:02X}AA", rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());
//         let mut objects = Vec::new();
//         let num_circles = rng.gen_range(20..=50);
//         for _ in 0..num_circles {
//             let x = rng.gen_range(-100.0..=100.0);
//             let y = rng.gen_range(-100.0..=100.0);
//             let r = rng.gen_range(-10.0..=20.0);
//             let circle = Circle { x, y, r };
//             objects.push(circle);
//         }
//         let layer = Layer { name, color, objects };
//         layers.push(layer);
//     }
//     layers
// }

// fn main() {
//     let seed = 42; 

    
//     let layers = gen_obj_layer_list(seed, 3);
//     for layer in &layers {
//         println!("Layer Name: {}, Color: {}", layer.name, layer.color);
//         for circle in &layer.objects {
//             println!("Circle (x: {}, y: {}, r: {})", circle.x, circle.y, circle.r);
//         }
//     }
// }



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_gen_obj_layer_list() {
        
//         let seed = 42;
    
//         let layers = gen_obj_layer_list(seed, 3);

//         assert_eq!(layers.len(), 3);


//         for layer in &layers {
//             let num_circles = layer.objects.len();
//             assert!(num_circles >= 20 && num_circles <= 50);
//         }
//     }
// }
// Q1.2)
// #[derive(Debug)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }

// #[derive(Debug)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }

// impl Layer {
//     fn average_area(&self) -> f64 {
//         let total_area: f64 = self.objects.iter().map(|circle| circle.area()).sum();
//         let num_circles = self.objects.len() as f64;
//         total_area / num_circles
//     }
// }

// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.r * self.r
//     }
// }

// fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
//     layers
//         .iter()
//         .map(|layer| (layer.name.clone(), layer.average_area()))
//         .collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_cal_average_area() {
//         let circle1 = Circle { x: 0.0, y: 0.0, r: 5.0 };
//         let circle2 = Circle { x: 0.0, y: 0.0, r: 10.0 };
//         let circle3 = Circle { x: 0.0, y: 0.0, r: 15.0 };

//         let layer1 = Layer {
//             name: String::from("Layer 1"),
//             color: String::from("#RRGGBBAA"),
//             objects: vec![circle1, circle2],
//         };

//         let layer2 = Layer {
//             name: String::from("Layer 2"),
//             color: String::from("#RRGGBBAA"),
//             objects: vec![circle3],
//         };

//         let layers = vec![layer1, layer2];

//         let average_areas = cal_average_area(&layers);

//         assert_eq!(average_areas.len(), 2);
//         assert_eq!(average_areas[0], ("Layer 1".to_string(), 196.34954084936209));
//         assert_eq!(average_areas[1], ("Layer 2".to_string(), 706.8583470577034)); 
//     }
// }

// fn main() {
    
// }

// Q2.1)
// use rand::{Rng, SeedableRng};
// use rand::rngs::SmallRng;
// use csv::Writer;

// #[derive(Debug, serde::Serialize)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }

// #[derive(Debug, serde::Serialize)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }

// fn gen_obj_layer_list(seed: u64, n: usize) -> Vec<Layer> {
//     let mut rng = SmallRng::seed_from_u64(seed);

//     let mut layers = Vec::new();
//     for i in 0..n {
//         let name = format!("Layer {}", i);
//         let color = format!("#{:02X}{:02X}{:02X}AA", rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());
//         let mut objects = Vec::new();
//         let num_circles = rng.gen_range(20..=50);
//         for _ in 0..num_circles {
//             let x = rng.gen_range(-100.0..=100.0);
//             let y = rng.gen_range(-100.0..=100.0);
//             let r = rng.gen_range(-10.0..=20.0);
//             let circle = Circle { x, y, r };
//             objects.push(circle);
//         }
//         let layer = Layer { name, color, objects };
//         layers.push(layer);
//     }
//     layers
// }

// fn main() {
//     let seed = 42; // Change this to your desired seed value
//     let n = 5;     // Change this to the number of layers you want

//     let layers = gen_obj_layer_list(seed, n);

//     // Define the path for the CSV file
//     let csv_file_path = "layers.csv";

//     // Create and open the CSV file for writing
//     let file = std::fs::File::create(csv_file_path).expect("Failed to create CSV file");
//     let mut writer = Writer::from_writer(file);

//     // Serialize and write the layers to the CSV file
//     for layer in &layers {
//         writer.serialize(layer).expect("Failed to write layer to CSV");
//     }

//     println!("CSV file saved as {}", csv_file_path);
// // }

// Q2.2)
// use csv::{Reader, Writer};
// use std::error::Error;
// use std::fs::File;

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }

// fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
//     layers
//         .iter()
//         .map(|layer| {
//             let average_area = layer
//                 .objects
//                 .iter()
//                 .map(|circle| circle.area())
//                 .sum::<f64>()
//                 / layer.objects.len() as f64;
//             (layer.name.clone(), average_area)
//         })
//         .collect()
// }

// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.r * self.r
//     }
// }
// fn main() -> Result<(), Box<dyn Error>> {
    
//     let csv_file_path = "layers.csv";
//     let file = File::open(csv_file_path)?;
//     let mut reader = Reader::from_reader(file);

//     let mut layers = Vec::new();
//     for result in reader.deserialize() {
//         let layer: Layer = result?;
//         layers.push(layer);
//     }
//     println!("Layers read from CSV: {:?}", layers);
//     let average_areas = cal_average_area(&layers);
//     println!("Average Areas: {:?}", average_areas);
//     let output_csv_file_path = "average_areas.csv";
//     let output_file = File::create(output_csv_file_path)?;
//     let mut writer = Writer::from_writer(output_file);

//     for (name, avg_area) in &average_areas {
//         writer.serialize((name, avg_area))?;
//     }
//     println!("Average areas saved as {}", output_csv_file_path);
//     Ok(())
// // }

// Q3.1)
// use std::error::Error;
// use std::fs::File;
// use std::io::Write;
// use csv::Reader;

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }
// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }
// fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
//     layers
//         .iter()
//         .map(|layer| {
//             let average_area = layer
//                 .objects
//                 .iter()
//                 .map(|circle| circle.area())
//                 .sum::<f64>()
//                 / layer.objects.len() as f64;
//             (layer.name.clone(), average_area)
//         })
//         .collect()
// }
// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.r * self.r
//     }
// }
// fn main() -> Result<(), Box<dyn Error>> {
//     let csv_file_path = "layers.csv";
//     let file = File::open(csv_file_path)?;
//     let mut reader = Reader::from_reader(file);
//     let mut layers = Vec::new();
//     for result in reader.deserialize() {
//         let layer: Layer = result?;
//         layers.push(layer);
//     }
//     println!("Layers read from CSV: {:?}", layers);
//     let average_areas = cal_average_area(&layers);
//     println!("Average Areas: {:?}", average_areas);
//     let output_html_file_path = "average_areas.html";
//     let mut html_file = File::create(output_html_file_path)?;
//     let mut html_content = String::new();
//     html_content.push_str("<html><body>\n");
//     html_content.push_str("<table border=\"1\">\n");
//     html_content.push_str("<tr><th>Layer Name</th><th>Average Area</th></tr>\n");

//     for (name, avg_area) in &average_areas {
//         html_content.push_str("<tr>");
//         html_content.push_str(&format!("<td>{}</td><td>{}</td>", name, avg_area));
//         html_content.push_str("</tr>\n");
//     }
//     html_content.push_str("</table>\n");
//     html_content.push_str("</body></html>\n");
//     write!(html_file, "{}", html_content)?;
//     println!("HTML table saved as {}", output_html_file_path);
//     Ok(())
// }


// Q3.2)
// use std::error::Error;
// use std::fs::File;
// use std::io::Write;
// use csv::Reader;

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Circle {
//     x: f64,
//     y: f64,
//     r: f64,
// }


// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Layer {
//     name: String,
//     color: String,
//     objects: Vec<Circle>,
// }
// #[derive(Debug)]
// struct LayerStats {
//     name: String,
//     average_area: f64,
//     min_area: f64,
//     max_area: f64,
// }


// fn calculate_layer_stats(layer: &Layer) -> LayerStats {
//     let average_area = layer
//         .objects
//         .iter()
//         .map(|circle| circle.area())
//         .sum::<f64>()
//         / layer.objects.len() as f64;
    
//     let min_area = layer
//         .objects
//         .iter()
//         .map(|circle| circle.area())
//         .fold(f64::INFINITY, f64::min);

//     let max_area = layer
//         .objects
//         .iter()
//         .map(|circle| circle.area())
//         .fold(f64::NEG_INFINITY, f64::max);

//     LayerStats {
//         name: layer.name.clone(),
//         average_area,
//         min_area,
//         max_area,
//     }
// }

// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.r * self.r
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let csv_file_path = "layers.csv";
//     let file = File::open(csv_file_path)?;
//     let mut reader = Reader::from_reader(file);

//     let mut layers = Vec::new();
//     for result in reader.deserialize() {
//         let layer: Layer = result?;
//         layers.push(layer);
//     }

//     println!("Layers read from CSV: {:?}", layers);

//     let layer_stats: Vec<LayerStats> = layers.iter().map(|layer| calculate_layer_stats(layer)).collect();

//     println!("Layer Stats: {:?}", layer_stats);

//     let output_html_file_path = "average_areas.html";
//     let mut html_file = File::create(output_html_file_path)?;

//     let mut html_content = String::new();
//     html_content.push_str("<html><body>\n");
//     html_content.push_str("<table border=\"1\">\n");
//     html_content.push_str("<tr><th>Layer Name</th><th>Average Area</th><th>Min Area</th><th>Max Area</th></tr>\n");

//     for stats in &layer_stats {
//         html_content.push_str("<tr>");
//         html_content.push_str(&format!("<td>{}</td><td>{}</td><td>{}</td><td>{}</td>", stats.name, stats.average_area, stats.min_area, stats.max_area));
//         html_content.push_str("</tr>\n");
//     }

//     html_content.push_str("</table>\n");
//     html_content.push_str("</body></html>\n");

//     write!(html_file, "{}", html_content)?;

//     println!("HTML table saved as {}", output_html_file_path);
//     Ok(())
// }
