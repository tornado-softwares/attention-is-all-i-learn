import { k_nn_classifier } from "./lib/classifier";
import { dataset } from "./lib/dataset";

async function main(){
    const training_dataset = new dataset({file_path: "/home/tornado-softwares/Bureau/tornado/data/handwritten-digits/train.jsonl"})
    const testing_dataset = new dataset({file_path: "/home/tornado-softwares/Bureau/tornado/data/handwritten-digits/test.jsonl"})
   
    console.time("[dataset] loading")
    await Promise.all([testing_dataset.load(),training_dataset.load()])
    console.timeEnd("[dataset] loading")
    
    const classifier = new k_nn_classifier({training_dataset, testing_dataset, k:3})

    console.time("[classifier] training")
    classifier.train()
    console.timeEnd("[classifier] training")

    console.time("[classifier] evaluation")
    classifier.evaluate()
    console.timeEnd("[classifier] evaluation")

}

main()