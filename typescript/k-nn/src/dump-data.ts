import { writeFileSync,mkdirSync } from "fs";
import { dataset } from "./lib/dataset";

async function main(){
    const training_dataset = new dataset({file_path: "/home/tornado-softwares/Bureau/tornado/data/handwritten-digits/train.jsonl"})
    await training_dataset.load()
    mkdirSync("./knn-dataset-dump", { recursive: true });
    let row_index =0
    for (const row of training_dataset.rows){
        const image_buffer = Buffer.from(row.image.b64,"base64")
         writeFileSync(`./knn-dataset-dump/label-${row.label}-row-${row_index}.png`, image_buffer);
        row_index++    
    }   
}

main()