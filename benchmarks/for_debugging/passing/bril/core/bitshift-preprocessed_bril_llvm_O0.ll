; ModuleID = '/var/folders/jw/f07sz9zx0wqck930wjllkpyr0000gn/T/.tmp7QW3Ye/postprocessed.ll'
source_filename = "stdin"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx13.0.0"

@.str = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%ld\00", align 1
@.str.3 = private unnamed_addr constant [9 x i8] c"[object]\00", align 1
@.str.4 = private unnamed_addr constant [33 x i8] c"error: expected %d args, got %d\0A\00", align 1

declare dso_local i32 @putchar(i32)

declare dso_local i32 @printf(ptr, ...)

declare dso_local void @exit(i32)

declare dso_local i64 @atol(ptr)

declare dso_local noalias ptr @malloc(i64)

declare dso_local void @free(ptr)

define dso_local i32 @btoi(ptr %0) {
  %2 = load i8, ptr %0, align 1
  %3 = icmp eq i8 %2, 116
  %4 = zext i1 %3 to i32
  ret i32 %4
}

define dso_local void @print_bool(i1 %0) {
  br i1 %0, label %2, label %4

2:                                                ; preds = %1
  %3 = call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str)
  br label %6

4:                                                ; preds = %1
  %5 = call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.1)
  br label %6

6:                                                ; preds = %4, %2
  ret void
}

define dso_local void @print_space() {
  %1 = call i32 @putchar(i32 32)
  ret void
}

define dso_local void @print_newline() {
  %1 = call i32 @putchar(i32 10)
  ret void
}

define dso_local void @print_int(i64 %0) {
  %2 = call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.2, i64 %0)
  ret void
}

define dso_local void @print_ptr(ptr %0) {
  %2 = call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.3)
  ret void
}

define dso_local i64 @__pow(i64 %x, i64 %n) {
pre_entry:
  %v3_0 = icmp eq i64 %n, 1
  br i1 %v3_0, label %then.0, label %else.0

then.0:                                           ; preds = %pre_entry
  ret i64 %x

else.0:                                           ; preds = %pre_entry
  %v8_0 = sdiv i64 %n, 2
  %half_0 = call i64 @__pow(i64 %x, i64 %v8_0)
  %v11_0 = mul i64 %half_0, %half_0
  %v15_0 = call i64 @__mod(i64 %n, i64 2)
  %v17_0 = icmp eq i64 %v15_0, 1
  br i1 %v17_0, label %then.12, label %else.12

then.12:                                          ; preds = %else.0
  %v20_0 = mul i64 %v11_0, %x
  br label %endif.12

else.12:                                          ; preds = %else.0
  br label %endif.12

endif.12:                                         ; preds = %else.12, %then.12
  %ans_2 = phi i64 [ %v11_0, %else.12 ], [ %v20_0, %then.12 ]
  ret i64 %ans_2
}

define dso_local i64 @__mod(i64 %a, i64 %b) {
pre_entry:
  %a.fr = freeze i64 %a
  %0 = srem i64 %a.fr, %b
  ret i64 %0
}

define dso_local i64 @__LEFTSHIFT(i64 %x, i64 %step) {
pre_entry:
  %p_0 = call i64 @__pow(i64 2, i64 %step)
  %v4_0 = mul i64 %p_0, %x
  ret i64 %v4_0
}

define dso_local i64 @__RIGHTSHIFT(i64 %x, i64 %step) {
pre_entry:
  %p_0 = call i64 @__pow(i64 2, i64 %step)
  %v4_0 = sdiv i64 %x, %p_0
  ret i64 %v4_0
}

define dso_local void @__main() {
b0:
  br label %loop_cond

loop_cond:                                        ; preds = %loop2_done, %b0
  %loop_counter_1 = phi i64 [ %loop_counter_2, %loop2_done ], [ 10, %b0 ]
  %loop_cond_0 = icmp slt i64 %loop_counter_1, 40
  br i1 %loop_cond_0, label %loop_body, label %loop_done

loop_body:                                        ; preds = %loop_cond
  br label %loop2_cond

loop2_cond:                                       ; preds = %loop3_done, %loop_body
  %loop2_counter_1 = phi i64 [ %loop2_counter_2, %loop3_done ], [ 10, %loop_body ]
  %loop2_cond_0 = icmp slt i64 %loop2_counter_1, 40
  br i1 %loop2_cond_0, label %loop2_body, label %loop2_done

loop2_body:                                       ; preds = %loop2_cond
  br label %loop3_cond

loop3_cond:                                       ; preds = %loop4_done, %loop2_body
  %loop3_counter_1 = phi i64 [ %loop3_counter_2, %loop4_done ], [ 10, %loop2_body ]
  %loop3_cond_0 = icmp slt i64 %loop3_counter_1, 40
  br i1 %loop3_cond_0, label %loop3_body, label %loop3_done

loop3_body:                                       ; preds = %loop3_cond
  br label %loop4_cond

loop4_cond:                                       ; preds = %loop4_body, %loop3_body
  %loop4_counter_1 = phi i64 [ %loop4_counter_2, %loop4_body ], [ 10, %loop3_body ]
  %loop4_cond_0 = icmp slt i64 %loop4_counter_1, 40
  br i1 %loop4_cond_0, label %loop4_body, label %loop4_done

loop4_body:                                       ; preds = %loop4_cond
  call void @__orig_main(i64 %loop_counter_1, i64 %loop2_counter_1, i64 %loop3_counter_1, i64 %loop4_counter_1)
  %loop4_counter_2 = add i64 %loop4_counter_1, 1
  br label %loop4_cond

loop4_done:                                       ; preds = %loop4_cond
  %loop3_counter_2 = add i64 %loop3_counter_1, 1
  br label %loop3_cond

loop3_done:                                       ; preds = %loop3_cond
  %loop2_counter_2 = add i64 %loop2_counter_1, 1
  br label %loop2_cond

loop2_done:                                       ; preds = %loop2_cond
  %loop_counter_2 = add i64 %loop_counter_1, 1
  br label %loop_cond

loop_done:                                        ; preds = %loop_cond
  ret void
}

define dso_local void @__orig_main(i64 %a, i64 %b, i64 %c, i64 %d) {
pre_entry:
  %ans1_0 = call i64 @__LEFTSHIFT(i64 %a, i64 %b)
  call void @print_int(i64 %ans1_0)
  call void @print_newline()
  %ans2_0 = call i64 @__RIGHTSHIFT(i64 %c, i64 %d)
  call void @print_int(i64 %ans2_0)
  call void @print_newline()
  ret void
}

define dso_local i32 @main(i32 %argc, ptr %argv) {
  %.not = icmp eq i32 %argc, 1
  br i1 %.not, label %4, label %1

1:                                                ; preds = %0
  %2 = add nsw i32 %argc, -1
  %3 = call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.4, i32 0, i32 %2)
  call void @exit(i32 2)
  unreachable

4:                                                ; preds = %0
  call void @__main()
  ret i32 0
}
