import type { Token, TokenADT } from "./Token"

export class Tokenizer {
    private tokens: Token[] = []

    private start = 0
    private current = 0
    private line = 0

    constructor(private readonly source: string) {}

    public scanTokens(): Token[] {
        while (!this.reachedEOF()) {
            this.start = this.current

            this.scanToken()
        }

        this.tokens.push({
            _type: "eof",
            lexeme: "",
            line: this.line
        })
        return this.tokens
    }

    private reachedEOF() {
        return this.current > this.source.length - 1
    }

    private scanToken() {
        const character = this.advance()

        switch (character) {
            case "(":
                this.addToken({
                    _type: "leftParen"
                })
                break
            case ")":
                this.addToken({
                    _type: "rightParen"
                })
                break
            case "{":
                this.addToken({
                    _type: "leftBrace"
                })
                break
            case "}":
                this.addToken({
                    _type: "rightBrace"
                })
                break
            case ",":
                this.addToken({
                    _type: "comma"
                })
                break
            case ".":
                this.addToken({
                    _type: "dot"
                })
                break
            case "-": {
                if (this.match("-")) {
                    while (
                        this.peek() !== "\n" &&
                        !this.reachedEOF()
                    ) {
                        this.advance()
                    }
                } else {
                    this.addToken({
                        _type: "minus"
                    })
                }
                break
            }
            case "+":
                this.addToken({
                    _type: "plus"
                })
                break
            case ";":
                this.addToken({
                    _type: "semicolon"
                })
                break
            case "*":
                this.addToken({
                    _type: "star"
                })
                break
            case "/":
                this.addToken({
                    _type: "slash"
                })
                break
            case "!": {
                const _type = this.match("=")
                    ? "bangEqual"
                    : "bang"
                this.addToken({ _type })
                break
            }
            case "=": {
                const _type = this.match("=")
                    ? "equalEqual"
                    : "equal"
                this.addToken({ _type })
                break
            }
            case "<": {
                const _type = this.match("=")
                    ? "lessEqual"
                    : "less"
                this.addToken({ _type })
                break
            }
            case ">": {
                const _type = this.match("=")
                    ? "greaterEqual"
                    : "greater"
                this.addToken({ _type })
                break
            }
            case " ":
            case "\r":
            case "\t":
                break
            case "\n":
                this.line++
                break
            case "0":
                this.leadingZeroNumber()
                break
            case "1":
            case "2":
            case "3":
            case "4":
            case "5":
            case "6":
            case "7":
            case "8":
            case "9":
                this.number()
                break
            case '"':
                this.string()
                break
            default:
                console.error(
                    `Unexpected character: "${character}"`
                )
                break
        }
    }

    private advance(advanceBy = 1) {
        this.current += advanceBy
        return this.source.charAt(this.current - 1)
    }

    private peek() {
        if (this.reachedEOF()) return ""

        return this.source.charAt(this.current)
    }

    private peekNext(peekBy = 1) {
        if (this.current + 1 >= this.source.length) {
            return ""
        }

        return this.source.charAt(this.current + peekBy)
    }

    private match(expected: string, lowercase = false) {
        if (this.reachedEOF()) return false

        const character = this.source.charAt(this.current)
        if (!lowercase && character !== expected) {
            return false
        } else if (
            lowercase &&
            character.toLowerCase() !== expected
        ) {
            return false
        }

        this.current++
        return true
    }

    private addToken(tokenMetadata: TokenADT) {
        const lexeme = this.source.slice(
            this.start,
            this.current
        )
        const { line } = this

        const token = {
            ...tokenMetadata,
            lexeme,
            line
        }

        this.tokens.push(token)
    }

    private isDigit(character: string) {
        return /[0-9]/.test(character)
    }
    private isBinaryDigit(character: string) {
        return character === "0" || character === "1"
    }
    private isOctalDigit(character: string) {
        return /[0-7]/.test(character)
    }
    private isHexadecimalDigit(character: string) {
        return /[0-9a-f]/i.test(character)
    }
    private number() {
        while (this.isDigit(this.peek())) this.advance()

        if (
            this.match(".") &&
            this.isDigit(this.peekNext())
        ) {
            this.advance()

            while (this.isDigit(this.peek())) this.advance()
        }
        if (this.match("e", true)) {
            if (this.isDigit(this.peekNext())) {
                this.advance()

                while (this.isDigit(this.peek())) {
                    this.advance()
                }
            } else if (
                this.peekNext() === "-" &&
                this.isDigit(this.peekNext(2))
            ) {
                this.advance(2)

                while (this.isDigit(this.peek())) {
                    this.advance()
                }
            }
        }

        const lexeme = this.source.slice(
            this.start,
            this.current
        )

        const bigInt = this.match("n")
        if (bigInt) this.advance()

        if (bigInt) {
            const literal = BigInt(lexeme)
            const kind = "decimal"

            return this.addToken({
                _type: "number",
                literal,
                kind,
                bigInt
            })
        }

        const literal = Number(lexeme)
        const kind = lexeme.toLowerCase().includes("e")
            ? "exponential"
            : "decimal"

        this.addToken({
            _type: "number",
            literal,
            kind,
            bigInt
        })
    }

    private leadingZeroNumber() {
        if (this.match("b", true)) {
            if (this.isBinaryDigit(this.peekNext())) {
                this.advance()

                while (this.isBinaryDigit(this.peek())) {
                    this.advance()
                }
            } else {
                console.error("Binary digit expected")
            }
        } else if (this.match("o", true)) {
            if (this.isOctalDigit(this.peekNext())) {
                this.advance()

                while (this.isOctalDigit(this.peek())) {
                    this.advance()
                }
            } else {
                console.error("Octal digit expected")
            }
        } else if (this.match("x", true)) {
            if (this.isHexadecimalDigit(this.peekNext())) {
                this.advance()

                while (
                    this.isHexadecimalDigit(this.peek())
                ) {
                    this.advance()
                }
            } else {
                console.error("Hexadecimal digit expected")
            }
        } else {
            return this.number()
        }

        const lexeme = this.source.slice(
            this.start,
            this.current
        )

        const bigInt = this.match("n")
        if (bigInt) this.advance()

        if (bigInt) {
            const literal = BigInt(lexeme)
            const [, secondChar] = lexeme.toLowerCase()
            const kind =
                secondChar === "b"
                    ? "binary"
                    : secondChar === "o"
                    ? "octal"
                    : "hexadecimal"

            return this.addToken({
                _type: "number",
                literal,
                kind,
                bigInt
            })
        }

        const literal = Number(lexeme)
        const [, secondChar] = lexeme.toLowerCase()
        const kind =
            secondChar === "b"
                ? "binary"
                : secondChar === "o"
                ? "octal"
                : "hexadecimal"

        this.addToken({
            _type: "number",
            literal,
            kind,
            bigInt
        })
    }

    private string() {
        while (this.peek() !== '"' && !this.reachedEOF()) {
            if (this.peek() === "\n") this.line++
            this.advance()
        }

        if (this.reachedEOF()) {
            console.error("Unterminated string literal.")
            return
        }

        this.advance()

        const literal = this.source.slice(
            this.start + 1,
            this.current - 1
        )

        this.addToken({ _type: "string", literal })
    }
}
