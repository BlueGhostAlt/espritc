import type { ADT } from "./types"

type NumberKind =
    | "decimal"
    | "exponential"
    | "binary"
    | "octal"

export type TokenADT = ADT<{
    leftParen: {}
    rightParen: {}
    leftBrace: {}
    rightBrace: {}
    comma: {}
    dot: {}
    minus: {}
    plus: {}
    semicolon: {}
    star: {}
    slash: {}
    eof: {}
    bangEqual: {}
    bang: {}
    equalEqual: {}
    equal: {}
    lessEqual: {}
    less: {}
    greaterEqual: {}
    greater: {}
    number: { literal: number; kind: NumberKind }
    string: { literal: string }
}>

export type Token = TokenADT & {
    readonly lexeme: string
    readonly line: number
}
