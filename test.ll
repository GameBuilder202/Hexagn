; ModuleID = 'hexagn'
source_filename = "hexagn"

declare i8 @putchar(i8 %0)

define i32 @somefunc() {
entry:
  ret i32 69420
}

define i8 @main() {
entry:
  %putchar = call i8 @putchar(i64 72)
  %putchar1 = call i8 @putchar(i64 101)
  %putchar2 = call i8 @putchar(i64 108)
  %putchar3 = call i8 @putchar(i64 108)
  %putchar4 = call i8 @putchar(i64 111)
  %putchar5 = call i8 @putchar(i64 32)
  %putchar6 = call i8 @putchar(i64 119)
  %putchar7 = call i8 @putchar(i64 111)
  %putchar8 = call i8 @putchar(i64 114)
  %putchar9 = call i8 @putchar(i64 108)
  %putchar10 = call i8 @putchar(i64 100)
  %putchar11 = call i8 @putchar(i64 33)
  %putchar12 = call i8 @putchar(i64 10)
  %somefunc = call i32 @somefunc()
  %buildvar = alloca i32, align 4
  store i32 69, ptr %buildvar, align 4


  ret i8 0
}

