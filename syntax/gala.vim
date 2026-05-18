syn match   galaComment "//.*$"
syn region  galaComment start="/\*" end="\*/"

syn region  galaString start="\"" skip="\\\"" end="\"" contains=galaEscape
syn match   galaEscape "\\."

syn keyword galaKeyword fn struct return if else
syn keyword galaType f16 f32 f64 u8 u16 u32 u64 i8 i16 i32 i64

syn match galaNumber "\v\d+"
syn match galaFunction "\v[a-zA-Z_]\w*\ze\s*\("

syn match galaOperator "[+\-*/=<>!&|]"

hi def link galaKeyword Keyword
hi def link galaType Type
hi def link galaNumber Number
hi def link galaFunction Function
hi def link galaString String
hi def link galaComment Comment
hi def link galaOperator Operator
