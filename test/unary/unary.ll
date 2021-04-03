; ModuleID = 'test/unary/unary.bc'
source_filename = "unary"

define i32 @neg_unary() {
neg_unary:
  %a = alloca i32, align 4
  store i32 -2, i32* %a, align 4
  %load_val = load i32, i32* %a, align 4
  ret i32 %load_val
}

define i32 @not_unary() {
not_unary:
  ret i32 -3
}

define i32 @complement_unary() {
complement_unary:
  ret i1 false
}
