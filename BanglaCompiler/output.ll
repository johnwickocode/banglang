; ModuleID = 'BanglaModule'
source_filename = "BanglaModule"

@0 = private unnamed_addr constant [4 x i8] c"\E0\A7\A6\00", align 1
@1 = private unnamed_addr constant [4 x i8] c"\E0\A7\A7\00", align 1
@2 = private unnamed_addr constant [4 x i8] c"\E0\A7\A8\00", align 1
@3 = private unnamed_addr constant [4 x i8] c"\E0\A7\A9\00", align 1
@4 = private unnamed_addr constant [4 x i8] c"\E0\A7\AA\00", align 1
@5 = private unnamed_addr constant [4 x i8] c"\E0\A7\AB\00", align 1
@6 = private unnamed_addr constant [4 x i8] c"\E0\A7\AC\00", align 1
@7 = private unnamed_addr constant [4 x i8] c"\E0\A7\AD\00", align 1
@8 = private unnamed_addr constant [4 x i8] c"\E0\A7\AE\00", align 1
@9 = private unnamed_addr constant [4 x i8] c"\E0\A7\AF\00", align 1
@10 = private unnamed_addr constant [2 x i8] c"-\00", align 1
@11 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1

declare i32 @printf(ptr, ...)

define void @print_bangla(i32 %0) {
entry:
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %zero_handle, label %non_zero

loop:                                             ; preds = %extract, %after_minus
  %2 = load i32, ptr %value, align 4
  %3 = icmp ne i32 %2, 0
  br i1 %3, label %extract, label %print

print:                                            ; preds = %loop
  %idx = alloca i32, align 4
  %4 = load i32, ptr %count, align 4
  %5 = sub i32 %4, 1
  store i32 %5, ptr %idx, align 4
  br label %print_loop

exit:                                             ; preds = %print_body, %print_loop, %zero_handle
  %6 = call i32 (ptr, ...) @printf(ptr @11)
  ret void

zero_handle:                                      ; preds = %entry
  %7 = call i32 (ptr, ...) @printf(ptr @0)
  br label %exit

non_zero:                                         ; preds = %entry
  %digits = alloca [12 x i32], align 4
  %count = alloca i32, align 4
  store i32 0, ptr %count, align 4
  %value = alloca i32, align 4
  %8 = icmp slt i32 %0, 0
  %9 = sub i32 0, %0
  %10 = select i1 %8, i32 %9, i32 %0
  store i32 %10, ptr %value, align 4
  br i1 %8, label %minus_str, label %after_minus

minus_str:                                        ; preds = %non_zero
  %11 = call i32 (ptr, ...) @printf(ptr @10)
  br label %after_minus

after_minus:                                      ; preds = %minus_str, %non_zero
  br label %loop

extract:                                          ; preds = %loop
  %12 = load i32, ptr %value, align 4
  %13 = srem i32 %12, 10
  %14 = sdiv i32 %12, 10
  store i32 %14, ptr %value, align 4
  %15 = load i32, ptr %count, align 4
  %16 = getelementptr [12 x i32], ptr %digits, i32 0, i32 %15
  store i32 %13, ptr %16, align 4
  %17 = add i32 %15, 1
  store i32 %17, ptr %count, align 4
  br label %loop

print_loop:                                       ; preds = %next9, %next8, %next7, %next6, %next5, %next4, %next3, %next2, %next1, %next0, %print
  %18 = load i32, ptr %idx, align 4
  %19 = icmp sge i32 %18, 0
  br i1 %19, label %print_body, label %exit

print_body:                                       ; preds = %print_loop
  %20 = load i32, ptr %idx, align 4
  %21 = getelementptr [12 x i32], ptr %digits, i32 0, i32 %20
  %22 = load i32, ptr %21, align 4
  switch i32 %22, label %exit [
    i32 0, label %case0
    i32 1, label %case1
    i32 2, label %case2
    i32 3, label %case3
    i32 4, label %case4
    i32 5, label %case5
    i32 6, label %case6
    i32 7, label %case7
    i32 8, label %case8
    i32 9, label %case9
  ]

case0:                                            ; preds = %print_body
  %23 = call i32 (ptr, ...) @printf(ptr @0)
  br label %next0

next0:                                            ; preds = %case0
  %24 = load i32, ptr %idx, align 4
  %25 = sub i32 %24, 1
  store i32 %25, ptr %idx, align 4
  br label %print_loop

case1:                                            ; preds = %print_body
  %26 = call i32 (ptr, ...) @printf(ptr @1)
  br label %next1

next1:                                            ; preds = %case1
  %27 = load i32, ptr %idx, align 4
  %28 = sub i32 %27, 1
  store i32 %28, ptr %idx, align 4
  br label %print_loop

case2:                                            ; preds = %print_body
  %29 = call i32 (ptr, ...) @printf(ptr @2)
  br label %next2

next2:                                            ; preds = %case2
  %30 = load i32, ptr %idx, align 4
  %31 = sub i32 %30, 1
  store i32 %31, ptr %idx, align 4
  br label %print_loop

case3:                                            ; preds = %print_body
  %32 = call i32 (ptr, ...) @printf(ptr @3)
  br label %next3

next3:                                            ; preds = %case3
  %33 = load i32, ptr %idx, align 4
  %34 = sub i32 %33, 1
  store i32 %34, ptr %idx, align 4
  br label %print_loop

case4:                                            ; preds = %print_body
  %35 = call i32 (ptr, ...) @printf(ptr @4)
  br label %next4

next4:                                            ; preds = %case4
  %36 = load i32, ptr %idx, align 4
  %37 = sub i32 %36, 1
  store i32 %37, ptr %idx, align 4
  br label %print_loop

case5:                                            ; preds = %print_body
  %38 = call i32 (ptr, ...) @printf(ptr @5)
  br label %next5

next5:                                            ; preds = %case5
  %39 = load i32, ptr %idx, align 4
  %40 = sub i32 %39, 1
  store i32 %40, ptr %idx, align 4
  br label %print_loop

case6:                                            ; preds = %print_body
  %41 = call i32 (ptr, ...) @printf(ptr @6)
  br label %next6

next6:                                            ; preds = %case6
  %42 = load i32, ptr %idx, align 4
  %43 = sub i32 %42, 1
  store i32 %43, ptr %idx, align 4
  br label %print_loop

case7:                                            ; preds = %print_body
  %44 = call i32 (ptr, ...) @printf(ptr @7)
  br label %next7

next7:                                            ; preds = %case7
  %45 = load i32, ptr %idx, align 4
  %46 = sub i32 %45, 1
  store i32 %46, ptr %idx, align 4
  br label %print_loop

case8:                                            ; preds = %print_body
  %47 = call i32 (ptr, ...) @printf(ptr @8)
  br label %next8

next8:                                            ; preds = %case8
  %48 = load i32, ptr %idx, align 4
  %49 = sub i32 %48, 1
  store i32 %49, ptr %idx, align 4
  br label %print_loop

case9:                                            ; preds = %print_body
  %50 = call i32 (ptr, ...) @printf(ptr @9)
  br label %next9

next9:                                            ; preds = %case9
  %51 = load i32, ptr %idx, align 4
  %52 = sub i32 %51, 1
  store i32 %52, ptr %idx, align 4
  br label %print_loop
}

define i32 @main() {
entry:
  %0 = call i32 @bl_add(i32 12, i32 30)
  %"\E0\A6\97" = alloca i32, align 4
  store i32 %0, ptr %"\E0\A6\97", align 4
  %1 = load i32, ptr %"\E0\A6\97", align 4
  call void @print_bangla(i32 %1)
  ret i32 0
}


declare i32 @bl_add(i32, i32)
