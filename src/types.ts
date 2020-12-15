export type ADT<T extends Record<string, {}>> = {
    [K in keyof T]: { _type: K } & T[K]
}[keyof T]
export type ADTMember<ADT, Type extends string> = Omit<
    Extract<ADT, { _type: Type }>,
    "_type"
>
