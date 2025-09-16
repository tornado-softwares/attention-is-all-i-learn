Install node packages : `pnpm i`

Transpile the typescript code to javascript `pnpm build`

Run this to run a simple prediction`node dist/predict.ts`

Run this to run an evaluation, `node dist/evaluate.ts`

Don't hesitate to change the code, be curious and make your own implementation with other datasets etc.


This is a 28x28 image classifier of numbers from 0 to 9 that works with the k-nearest neighbor algorithm.

The training data is provided in the ./data folder. Don't forget to change the paths because I used absolute paths to annoy you. ^^

Other comments can be found in the code...
