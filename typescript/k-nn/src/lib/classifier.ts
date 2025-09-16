import { assert } from "console";
import { PNG } from "pngjs";
import { k_nn_inputs, neighbor } from "../types/classifier";
import { dataset } from "./dataset";
import fs from "fs"
export class k_nn_classifier {
  vectors_per_label: Record<string, vector[]> = {}
  training_dataset: dataset
  testing_dataset: dataset
  k: number 

  get is_loaded() { return Object.keys(this.vectors_per_label).length < 0 }

  constructor(settings: k_nn_inputs) {
    this.k = settings.k
    if (!settings.testing_dataset.is_loaded) throw new Error("[classifier] Testing dataset not pre-loaded.")
    if (!settings.training_dataset.is_loaded) throw new Error("[classifier] Training dataset not pre-loaded.")
    this.training_dataset = settings.training_dataset
    this.testing_dataset = settings.testing_dataset
  }

  evaluate() {
    let errors = 0
    let predictions = 0
    for (const row of this.testing_dataset.rows) {
      const image_buffer = Buffer.from(row.image.b64, "base64")
      const vector = this.image_buffer_to_vector(image_buffer)
      const nn = this.get_nearest_neighbors(vector, this.k);
      const label = this.choose_label(nn);
      if (label != row.label.toString())  errors += 1
      predictions += 1
      const accuracy = ((predictions - errors) / predictions * 100).toFixed(2)
      console.log("[classifier]", `Evaluating.. Errors :${errors} Predictions : ${predictions} Accuracy : ${accuracy}%`);
    }
    const accuracy = ((predictions - errors) / predictions * 100).toFixed(2)
    return { accuracy, predictions, errors }
  }

  train() {
    console.log("[classifier]", "Training, generating data samples vectors.")
    for (const row of this.training_dataset.rows) {
      const image_buffer = Buffer.from(row.image.b64, "base64")
      const vector = this.image_buffer_to_vector(image_buffer)
      if (this.vectors_per_label?.[row.label]) {
        this.vectors_per_label[row.label].push(vector)
      } else {
        this.vectors_per_label[row.label] = [vector]
      }
    }
  }

  predict(file_path: string) {
    if (this.is_loaded) throw new Error("[classifier] No loaded/trained vectors.")
    const image_buffer = fs.readFileSync(file_path);
    const vector = this.image_buffer_to_vector(image_buffer);
    const nn = this.get_nearest_neighbors(vector, this.k);
    const label = this.choose_label(nn);
    return {
      training_data: {
        labels:this.training_dataset.labels,
        count: this.training_dataset.rows.length
      },
      prediction: {
        k: this.k,
        nn,
        output: label
      }

    };
  }

  euclidean_distance(v1: number[], v2: number[]): number {
    let distance_square = 0;
    for (let i = 0; i < v1.length; i++) distance_square += (v1[i] - v2[i]) ** 2;
    return Math.sqrt(distance_square);
  }

  get_nearest_neighbors(vector: number[], k: number) {
    const distances: { label: string, distance: number }[] = [];
    for (const label in this.vectors_per_label) {
      for (const label_vector of this.vectors_per_label[label]) {
        distances.push({ label, distance: this.euclidean_distance(vector, label_vector) });
      }
    }
    distances.sort((a, b) => a.distance - b.distance);
    return distances.slice(0, k);
  }

  choose_label(neighbors: neighbor[]): string {
    const counts: Record<string, number> = {};
    for (const n of neighbors) counts[n.label] = (counts[n.label] || 0) + 1;
    let count = 0;
    let label = "";
    for (const am_i_the_super_label_oh_my_god in counts) {
      if (counts[label] > count) {
        count = counts[label];
        label = am_i_the_super_label_oh_my_god;
      }
    }
 
    return label;
  }

  // generated this function with chatgpt :( 
  image_buffer_to_vector(image_buffer: Buffer<ArrayBuffer>): vector {
    const png = PNG.sync.read(image_buffer);
    const vector: number[] = [];
    for (let y = 0; y < png.height; y++) {
      for (let x = 0; x < png.width; x++) {
        const idx = (png.width * y + x) << 2;
        const r = png.data[idx];
        const g = png.data[idx + 1];
        const b = png.data[idx + 2];
        const gray = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
        vector.push(gray);
      }
    }
    return vector
  }

}
