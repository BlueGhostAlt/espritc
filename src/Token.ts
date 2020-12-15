import type { ADT } from "./types"

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
    string: { literal: string }
    number: { literal: number }
}>

export type Token = TokenADT & {
    readonly lexeme: string
    readonly line: number
}
