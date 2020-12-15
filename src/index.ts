import { readFile } from "fs/promises"

import { Tokenizer } from "./Tokenizer"

const input = await readFile("examples/main.es", "utf-8")

const run = (source: string) => {
    const scanner = new Tokenizer(source)
    const tokens = scanner.scanTokens()

    for (const token of tokens) {
        console.log(token)
    }
}

run(input)
