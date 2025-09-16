import { dataset } from "../lib/dataset"

declare interface k_nn_inputs {
    k:number
    training_dataset:dataset
    testing_dataset:dataset
}

declare type neighbor = { label: string, distance: number }