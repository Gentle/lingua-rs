/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Writes the language models of all languages enabled at compile time into
//! a single blob file for use with the `external-models` feature:
//!
//!     cargo run --release --bin pack_models --features pack-models -- lingua-models.blob

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "lingua-models.blob".to_string());
    lingua::write_models_blob(&output_path).expect("failed to write models blob");
    let size = std::fs::metadata(&output_path).unwrap().len();
    println!("wrote {output_path} ({size} bytes)");
}
