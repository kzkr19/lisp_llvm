(print (+ 1 2))
(define (f n) (+ n 1))
(define (fact n)
  (if (= n 0)
      n
      (* n (fact (- n 1)))))

(define (fib n)
  (if (or (= n 0) (= n 1))
      1
      (+ (fib (- n 1) (- n 2)))))

(define (curry f)
  (lambda x (lambda y (apply f (append x y)))))

(define (map f xs)
  (if (null? xs)
      '()
      (cons (f (car xs)) (map f (cdr xs)))))

(define (filter p xs)
  (cond ((null? xs) '())
        ((p (car xs)) (cons (car xs) (filter p (cdr xs))))
        (else (filter p (cdr xs)))))

(define test-list 
  (list #\a #\b #\space 25 +58 #b10 #o10 #d10 #x10 #xFF #xff
        "string" "st" "adaf\"\\" #f #t))