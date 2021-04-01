; ModuleID = 'test/basic/basic.bc'
source_filename = "basic"

define i32 @main() {
main:
  %a = alloca i32, align 4
  store i32 1, i32* %a, align 4
  %b = alloca i32, align 4
  store i32 2, i32* %b, align 4
  %c = alloca i32, align 4
  %d = alloca i32, align 4
  ret i32 0
}
