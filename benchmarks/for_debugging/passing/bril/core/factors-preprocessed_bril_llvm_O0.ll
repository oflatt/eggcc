; ModuleID = '/var/folders/jw/f07sz9zx0wqck930wjllkpyr0000gn/T/.tmpWXUpvX/postprocessed.ll'
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

define dso_local void @__main() {
b0:
  br label %loop_cond

loop_cond:                                        ; preds = %loop_body, %b0
  %loop_counter_1 = phi i64 [ %loop_counter_2, %loop_body ], [ 10, %b0 ]
  %loop_cond_0 = icmp slt i64 %loop_counter_1, 100000
  br i1 %loop_cond_0, label %loop_body, label %loop_done

loop_body:                                        ; preds = %loop_cond
  call void @__orig_main(i64 %loop_counter_1)
  %loop_counter_2 = add i64 %loop_counter_1, 1
  br label %loop_cond

loop_done:                                        ; preds = %loop_cond
  ret void
}

define dso_local void @__orig_main(i64 %num) {
pre_entry:
  %num.fr = freeze i64 %num
  br label %loopcheck.outer

loopcheck.outer:                                  ; preds = %pre_entry, %ifno
  %fac_1.ph = phi i64 [ 2, %pre_entry ], [ %fac_2, %ifno ]
  %num_1.ph = phi i64 [ %num.fr, %pre_entry ], [ %num_1.lcssa1, %ifno ]
  br label %loopcheck

loopcheck:                                        ; preds = %loopcheck.outer, %ifyes
  %num_1 = phi i64 [ %num_2, %ifyes ], [ %num_1.ph, %loopcheck.outer ]
  %ispos_0 = icmp sgt i64 %num_1, 1
  br i1 %ispos_0, label %loopbody, label %loopend

loopbody:                                         ; preds = %loopcheck
  %0 = srem i64 %num_1, %fac_1.ph
  %iszero_0 = icmp eq i64 %0, 0
  br i1 %iszero_0, label %ifyes, label %ifno

ifyes:                                            ; preds = %loopbody
  call void @print_int(i64 %fac_1.ph)
  call void @print_newline()
  %num_2 = sdiv i64 %num_1, %fac_1.ph
  br label %loopcheck

ifno:                                             ; preds = %loopbody
  %num_1.lcssa1 = phi i64 [ %num_1, %loopbody ]
  %fac_2 = add i64 %fac_1.ph, 1
  br label %loopcheck.outer

loopend:                                          ; preds = %loopcheck
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
