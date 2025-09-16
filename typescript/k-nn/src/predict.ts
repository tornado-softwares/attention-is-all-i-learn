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

    console.time("[classifier] prediction")
    const prediction = classifier.predict("/home/tornado-softwares/Bureau/tornado/data/handwritten-digits/predict-me.png")
    console.log(prediction)
    console.timeEnd("[classifier] prediction") 


    // predictions are not really accurate if there are no grayscale levels in your input image, like if it's just white on black
    // example with : cant-predict-me.png
    // Why ? Well, because our training data contains only grayscale representations !
    // example run "pnpm build" then "node dist/dump-data.js" 
    // A folder with the training images will be created. Click on one at random and you will see the shades of gray/white!
}

main()