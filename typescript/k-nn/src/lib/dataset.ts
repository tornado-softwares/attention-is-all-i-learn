import * as fs from "fs/promises";

export class dataset {
    readonly filepath: string
    rows: dataset_row[] = []
    labels: Record<string,number> = {}

    get is_loaded(){return this.rows.length > 0}

    constructor(inputs: dataset_inputs) {
        this.filepath = inputs.file_path
    }

    async load() {
        console.log("[dataset]", `Reading and parsing dataset ${this.filepath}`)
        if (this.is_loaded) return this.rows
        const content = await fs.readFile(this.filepath, "utf-8");
        const lines = content.split("\n");

        for (const line of lines) {
            const cleaned_line = line.trim()
            try {
                const row:dataset_row = JSON.parse(cleaned_line)
                if(this.labels?.[row.label]){
                    this.labels[row.label] +=1
                }else{
                    this.labels[row.label] =1
                }
                this.rows.push(row)
            } catch (err) { }
        }

        return this.rows;
    }
}