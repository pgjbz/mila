pub enum Precedence {
    Lowest = 1,
    _AndOr = 2,
    _Equals = 3,
    _LessGreater = 4,
    _Sum = 5,
    _Product = 6,
    Prefix = 7,
    _Call = 8,
    _Index = 9,
}
