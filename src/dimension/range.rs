// use serde::{Deserialize, Serialize};
//
// #[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum NumberProviderType {
//     #[serde(rename = "minecraft:uniform")]
//     Uniform {
//         value: NumberProvider
//     },
//
//     #[serde(rename = "minecraft:constant")]
//     Constant {
//         value: NumberProvider
//     },
//
//     #[serde(rename = "minecraft:biased_to_bottom")]
//     BiasedToBottom {
//         value: NumberProvider
//     },
// }
//
// #[derive(Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum NumberProvider {
//     IntRange {
//         min_inclusive: i32,
//         max_inclusive: i32,
//     },
//     FloatRange {
//         min_inclusive: f32,
//         max_inclusive: f32,
//     },
//     Int(i32),
//     Float(f32),
// }
//
