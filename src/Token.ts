import type { ADT } from "./types"

type NumberKind =
    | "decimal"
    | "exponential"
    | "binary"
    | "octal"
    | "hexadecimal"

export type TokenADT = ADT<{
    bracket: {}
    operator: {}
    punctuation: {}
    eof: {}
    number:
        | {
              literal: number
              kind: NumberKind
              bigInt: false
          }
        | {
              literal: bigint
              kind: NumberKind
              bigInt: true
          }
    string: { literal: string }
}>

export type Token = TokenADT & {
    readonly lexeme: string
    readonly line: number
}
