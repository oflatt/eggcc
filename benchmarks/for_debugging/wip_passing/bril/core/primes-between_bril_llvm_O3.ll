; ModuleID = '/var/folders/jw/f07sz9zx0wqck930wjllkpyr0000gn/T/.tmp6ssiqu/compile.ll'
source_filename = "stdin"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx13.0.0"

@.str = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%ld\00", align 1
@.str.3 = private unnamed_addr constant [9 x i8] c"[object]\00", align 1
@.str.4 = private unnamed_addr constant [33 x i8] c"error: expected %d args, got %d\0A\00", align 1

; Function Attrs: nofree nounwind
declare dso_local noundef i32 @putchar(i32 noundef) local_unnamed_addr #0

; Function Attrs: nofree nounwind
declare dso_local noundef i32 @printf(i8* nocapture noundef readonly, ...) local_unnamed_addr #0

declare dso_local void @exit(i32) local_unnamed_addr

; Function Attrs: argmemonly mustprogress nofree norecurse nosync nounwind readonly willreturn
define dso_local i32 @btoi(i8* nocapture readonly %0) local_unnamed_addr #1 {
  %2 = load i8, i8* %0, align 1
  %3 = icmp eq i8 %2, 116
  %4 = zext i1 %3 to i32
  ret i32 %4
}

; Function Attrs: nofree nounwind
define dso_local void @print_bool(i1 %0) local_unnamed_addr #0 {
  %. = select i1 %0, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str.1, i64 0, i64 0)
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) %.)
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @print_space() local_unnamed_addr #0 {
  %1 = tail call i32 @putchar(i32 32)
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @print_newline() local_unnamed_addr #0 {
  %1 = tail call i32 @putchar(i32 10)
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @print_int(i64 %0) local_unnamed_addr #0 {
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0), i64 %0)
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @print_ptr(i8* nocapture readnone %0) local_unnamed_addr #0 {
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([9 x i8], [9 x i8]* @.str.3, i64 0, i64 0))
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @__main() local_unnamed_addr #0 {
b0:
  br label %inner_cond.preheader

inner_cond.preheader:                             ; preds = %b0, %inner_done
  %loop_counter_13 = phi i64 [ 10, %b0 ], [ %loop_counter_2, %inner_done ]
  br label %inner_body

inner_body:                                       ; preds = %inner_cond.preheader, %__orig_main.exit
  %inner_counter_11 = phi i64 [ 10, %inner_cond.preheader ], [ %inner_counter_2, %__orig_main.exit ]
  %t3_0.not3.i = icmp ugt i64 %loop_counter_13, %inner_counter_11
  br i1 %t3_0.not3.i, label %__orig_main.exit, label %for.inner.body.preheader.i

for.inner.body.preheader.i:                       ; preds = %inner_body, %if.outer.end.i
  %t2_24.i = phi i64 [ %t2_3.i, %if.outer.end.i ], [ %loop_counter_13, %inner_body ]
  %t8_067.i = lshr i64 %t2_24.i, 1
  br label %for.inner.body.i

for.inner.cond.i:                                 ; preds = %for.inner.body.i
  %t6_2.i = add nuw nsw i64 %t6_12.i, 1
  %exitcond.i = icmp eq i64 %t6_12.i, %t8_067.i
  br i1 %exitcond.i, label %if.outer.body.i, label %for.inner.body.i

for.inner.body.i:                                 ; preds = %for.inner.cond.i, %for.inner.body.preheader.i
  %t6_12.i = phi i64 [ %t6_2.i, %for.inner.cond.i ], [ 2, %for.inner.body.preheader.i ]
  %.urem = urem i64 %t2_24.i, %t6_12.i
  %t12_0.i = icmp eq i64 %.urem, 0
  br i1 %t12_0.i, label %if.outer.end.i, label %for.inner.cond.i

if.outer.body.i:                                  ; preds = %for.inner.cond.i
  %0 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0), i64 %t2_24.i) #5
  %1 = tail call i32 @putchar(i32 10) #5
  br label %if.outer.end.i

if.outer.end.i:                                   ; preds = %for.inner.body.i, %if.outer.body.i
  %t2_3.i = add nuw nsw i64 %t2_24.i, 1
  %t3_0.not.i.not = icmp ult i64 %t2_24.i, %inner_counter_11
  br i1 %t3_0.not.i.not, label %for.inner.body.preheader.i, label %__orig_main.exit

__orig_main.exit:                                 ; preds = %if.outer.end.i, %inner_body
  %inner_counter_2 = add nuw nsw i64 %inner_counter_11, 1
  %exitcond.not = icmp eq i64 %inner_counter_2, 400
  br i1 %exitcond.not, label %inner_done, label %inner_body

inner_done:                                       ; preds = %__orig_main.exit
  %loop_counter_2 = add nuw nsw i64 %loop_counter_13, 1
  %exitcond4.not = icmp eq i64 %loop_counter_2, 400
  br i1 %exitcond4.not, label %loop_done, label %inner_cond.preheader

loop_done:                                        ; preds = %inner_done
  ret void
}

; Function Attrs: nofree nounwind
define dso_local void @__orig_main(i64 %a, i64 %b) local_unnamed_addr #0 {
pre_entry:
  %0 = tail call i64 @llvm.smax.i64(i64 %a, i64 2)
  %t3_0.not3 = icmp sgt i64 %0, %b
  br i1 %t3_0.not3, label %for.outer.end, label %for.inner.cond.preheader

for.inner.cond.preheader:                         ; preds = %pre_entry, %if.outer.end
  %t2_24 = phi i64 [ %t2_3, %if.outer.end ], [ %0, %pre_entry ]
  %t9_0.not1 = icmp slt i64 %t2_24, 4
  br i1 %t9_0.not1, label %if.outer.body, label %for.inner.body.preheader

for.inner.body.preheader:                         ; preds = %for.inner.cond.preheader
  %t8_067 = lshr i64 %t2_24, 1
  br label %for.inner.body

for.inner.cond:                                   ; preds = %for.inner.body
  %t6_2 = add nuw nsw i64 %t6_12, 1
  %exitcond = icmp eq i64 %t6_12, %t8_067
  br i1 %exitcond, label %if.outer.body, label %for.inner.body

for.inner.body:                                   ; preds = %for.inner.body.preheader, %for.inner.cond
  %t6_12 = phi i64 [ %t6_2, %for.inner.cond ], [ 2, %for.inner.body.preheader ]
  %1 = srem i64 %t2_24, %t6_12
  %t12_0 = icmp eq i64 %1, 0
  br i1 %t12_0, label %if.outer.end, label %for.inner.cond

if.outer.body:                                    ; preds = %for.inner.cond, %for.inner.cond.preheader
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0), i64 %t2_24) #5
  %3 = tail call i32 @putchar(i32 10) #5
  br label %if.outer.end

if.outer.end:                                     ; preds = %for.inner.body, %if.outer.body
  %t2_3 = add i64 %t2_24, 1
  %t3_0.not = icmp sgt i64 %t2_3, %b
  br i1 %t3_0.not, label %for.outer.end, label %for.inner.cond.preheader

for.outer.end:                                    ; preds = %if.outer.end, %pre_entry
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define dso_local i64 @__mod(i64 %a, i64 %b) local_unnamed_addr #2 {
pre_entry:
  %0 = srem i64 %a, %b
  ret i64 %0
}

define dso_local i32 @main(i32 %argc, i8** nocapture readnone %argv) local_unnamed_addr {
  %1 = add nsw i32 %argc, -1
  %.not = icmp eq i32 %1, 0
  br i1 %.not, label %inner_cond.preheader.i, label %codeRepl

codeRepl:                                         ; preds = %0
  call void @main.cold.1(i32 %1) #6
  ret i32 0

inner_cond.preheader.i:                           ; preds = %0, %inner_done.i
  %loop_counter_13.i = phi i64 [ %loop_counter_2.i, %inner_done.i ], [ 10, %0 ]
  br label %inner_body.i

inner_body.i:                                     ; preds = %__orig_main.exit.i, %inner_cond.preheader.i
  %indvars.iv = phi i64 [ %indvars.iv.next, %__orig_main.exit.i ], [ 11, %inner_cond.preheader.i ]
  %inner_counter_11.i = phi i64 [ %inner_counter_2.i, %__orig_main.exit.i ], [ 10, %inner_cond.preheader.i ]
  %t3_0.not3.i.i = icmp ugt i64 %loop_counter_13.i, %inner_counter_11.i
  br i1 %t3_0.not3.i.i, label %__orig_main.exit.i, label %for.inner.body.preheader.i.i

for.inner.body.preheader.i.i:                     ; preds = %inner_body.i, %if.outer.end.i.i
  %t2_24.i.i = phi i64 [ %t2_3.i.i, %if.outer.end.i.i ], [ %loop_counter_13.i, %inner_body.i ]
  %t8_067.i.i = lshr i64 %t2_24.i.i, 1
  br label %for.inner.body.i.i

for.inner.cond.i.i:                               ; preds = %for.inner.body.i.i
  %t6_2.i.i = add nuw nsw i64 %t6_12.i.i, 1
  %exitcond.i.i = icmp eq i64 %t6_12.i.i, %t8_067.i.i
  br i1 %exitcond.i.i, label %if.outer.body.i.i, label %for.inner.body.i.i

for.inner.body.i.i:                               ; preds = %for.inner.cond.i.i, %for.inner.body.preheader.i.i
  %t6_12.i.i = phi i64 [ %t6_2.i.i, %for.inner.cond.i.i ], [ 2, %for.inner.body.preheader.i.i ]
  %.urem.i = urem i64 %t2_24.i.i, %t6_12.i.i
  %t12_0.i.i = icmp eq i64 %.urem.i, 0
  br i1 %t12_0.i.i, label %if.outer.end.i.i, label %for.inner.cond.i.i

if.outer.body.i.i:                                ; preds = %for.inner.cond.i.i
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0), i64 %t2_24.i.i) #5
  %3 = tail call i32 @putchar(i32 10) #5
  br label %if.outer.end.i.i

if.outer.end.i.i:                                 ; preds = %for.inner.body.i.i, %if.outer.body.i.i
  %t2_3.i.i = add nuw nsw i64 %t2_24.i.i, 1
  %exitcond.not = icmp eq i64 %t2_3.i.i, %indvars.iv
  br i1 %exitcond.not, label %__orig_main.exit.i, label %for.inner.body.preheader.i.i

__orig_main.exit.i:                               ; preds = %if.outer.end.i.i, %inner_body.i
  %inner_counter_2.i = add nuw nsw i64 %inner_counter_11.i, 1
  %exitcond.not.i = icmp eq i64 %inner_counter_2.i, 400
  %indvars.iv.next = add nuw nsw i64 %indvars.iv, 1
  br i1 %exitcond.not.i, label %inner_done.i, label %inner_body.i

inner_done.i:                                     ; preds = %__orig_main.exit.i
  %loop_counter_2.i = add nuw nsw i64 %loop_counter_13.i, 1
  %exitcond4.not.i = icmp eq i64 %loop_counter_2.i, 400
  br i1 %exitcond4.not.i, label %__main.exit, label %inner_cond.preheader.i

__main.exit:                                      ; preds = %inner_done.i
  ret i32 0
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i64 @llvm.smax.i64(i64, i64) #3

; Function Attrs: cold minsize noreturn
define internal void @main.cold.1(i32 %0) #4 {
newFuncRoot:
  br label %1

1:                                                ; preds = %newFuncRoot
  %2 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([33 x i8], [33 x i8]* @.str.4, i64 0, i64 0), i32 0, i32 %0)
  tail call void @exit(i32 2)
  unreachable
}

attributes #0 = { nofree nounwind }
attributes #1 = { argmemonly mustprogress nofree norecurse nosync nounwind readonly willreturn }
attributes #2 = { mustprogress nofree norecurse nosync nounwind readnone willreturn }
attributes #3 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #4 = { cold minsize noreturn }
attributes #5 = { nounwind }
attributes #6 = { noinline }
