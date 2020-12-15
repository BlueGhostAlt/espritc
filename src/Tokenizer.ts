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

    private advance() {
        this.current++
        return this.source.charAt(this.current - 1)
    }

    private peek() {
        if (this.reachedEOF()) return ""

        return this.source.charAt(this.current)
    }

    private match(expected: string) {
        if (this.reachedEOF()) return false
        if (this.source.charAt(this.current) !== expected) {
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
