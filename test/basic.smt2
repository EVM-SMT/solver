(declare-fun foo (Bool) Bool)
(declare-const a Bool)
(declare-const b Bool)
(assert (& a (foo b)))
(check-sat)
