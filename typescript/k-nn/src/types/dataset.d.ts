declare type dataset_row = {
    image:{
        b64:string 
    },
    label:number
}

declare interface dataset_inputs {
    file_path:string
 }