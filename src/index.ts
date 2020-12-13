import { add } from "./add"

const one = Promise.resolve(1)

console.log(
    `Hello from esprit! PI = ${add(await one, 2.14)}`
)
