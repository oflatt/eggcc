#[test]
fn simple_types() -> Result<(), egglog::Error> {
    let build = &*format!(
        "
        (let id1 (Id (i64-fresh!)))
        (let id2 (Id (i64-fresh!)))
        (let n (Add (Num id1 1) (Num id2 2)))
        (let m (Mul n n))
        (let s (Sub n m))
        (let x (LessThan m n))
        (let y (Not x))
        (let z (And x (Or y y)))
        (HasTypeDemand s)
        (HasTypeDemand z)
        "
    );
    let check = "
    (run-schedule (saturate type-analysis))

    (check (HasType n (IntT)))
    (check (HasType m (IntT)))
    (check (HasType s (IntT)))
    (check (HasType x (BoolT)))
    (check (HasType y (BoolT)))
    (check (HasType z (BoolT)))
    ";
    crate::run_test(build, check)
}

#[test]
fn switch() -> Result<(), egglog::Error> {
    let build = "
  (let b1 (Boolean (Id (i64-fresh!)) true))
  (let n1 (Num (Id (i64-fresh!)) 1))
  (let n2 (Num (Id (i64-fresh!)) 3))
  (let switch
    (Switch (Not (LessThan n1 n2))
            (Cons (Add n1 n1) (Cons (Sub n1 n2) (Cons (Mul n2 n2) (Nil))))))
  (HasTypeDemand switch)
  ";
    let check = "
  (run-schedule (saturate type-analysis))

  (check (HasType switch (IntT)))
  ";
    crate::run_test(build, check)
}

#[test]
fn tuple() -> Result<(), egglog::Error> {
  let build = 
  "
  (let n (Add (Num (Id (i64-fresh!)) 1) (Num (Id (i64-fresh!)) 2)))
        (let m (Mul n n))
        (let s (Sub n m))
        (let x (LessThan m n))
        (let y (Not x))
        (let z (And x (Or y y)))
  
  (let tup1 (All (Sequential) (Nil)))
  (let tup2 (All (Sequential) (Cons z (Nil))))
  (let tup3 (All (Parallel) (Cons x (Cons m (Nil)))))
  (let tup4 (All (Parallel) (Cons tup2 (Cons tup3 (Nil)))))
  (HasTypeDemand tup1)
  (HasTypeDemand tup2)
  (HasTypeDemand tup3)
  (HasTypeDemand tup4)

  (let get1 (Get tup3 0))
  (let get2 (Get tup3 1))
  (let get3 (Get (Get tup4 1) 1))
  (HasTypeDemand get1)
  (HasTypeDemand get2)
  (HasTypeDemand get3)
  ";
  let check =
  "
  (run-schedule (saturate type-analysis))
  (check (HasType tup1 (TupleT (TNil))))
  (check (HasType tup2 (TupleT (TCons (BoolT) (TNil)))))
  (check (HasType tup3 (TupleT (TCons (BoolT) (TCons (IntT) (TNil))))))
  (check (HasType tup4
    (TupleT (TCons (TupleT (TCons (BoolT) (TNil)))
    (TCons (TupleT (TCons (BoolT) (TCons (IntT) (TNil))))
          (TNil))))))

  
  (check (HasType get1 (BoolT)))
  (check (HasType get2 (IntT)))
  (check (HasType get3 (IntT)))
  ";
  crate::run_test(build, check)
}