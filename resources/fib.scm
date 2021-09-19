(define (fib n)
  (if (or (= n 0) (= n 1))
      1
      (+ (fib (- n 1)) (fib (- n 2)))))

(define (show-fib i max)
    (if (= i max)
        #f
        (begin 
            (display (fib i))
            (display #\newline)
            (show-fib (+ i 1) max))))

(show-fib 0 5)