;; Driver for the Earley and nboyer benchmarks.
(import (scheme base) (scheme cxr) (scheme process-context) (scheme file)
        (hoot ffi))

(define-foreign bench-start "bench" "start" -> none)
(define-foreign bench-end "bench" "end" -> none)

(define (fatal-error . args)
  (exit 1))

;; Parses the lemma table at run time: as a quoted constant it would be built
;; by one huge start function that dominates Wasm compile time.
(define (string->sexp s)
  (define (space? c) (memv c '(#\space #\newline #\tab)))
  ;; Returns the datum at the front of l and the rest of l.
  (define (parse l)
    (cond ((space? (car l)) (parse (cdr l)))
          ((char=? (car l) #\()
           (let loop ((l (cdr l)) (acc '()))
             (cond ((space? (car l)) (loop (cdr l) acc))
                   ((char=? (car l) #\)) (values (reverse acc) (cdr l)))
                   (else (call-with-values (lambda () (parse l))
                           (lambda (x l) (loop l (cons x acc))))))))
          (else
           (let loop ((l l) (tok '()))
             (if (or (null? l) (space? (car l)) (memv (car l) '(#\( #\))))
                 (let ((tok (list->string (reverse tok))))
                   (values (or (string->number tok) (string->symbol tok)) l))
                 (loop (cdr l) (cons (car l) tok)))))))
  (call-with-values (lambda () (parse (string->list s))) (lambda (x l) x)))

(include "earley.scm")
(include "nboyer.scm")

;; Same test as Octane's earley-boyer: count the parses of k `a`s under the
;; ambiguous grammar s -> a | s s (the Catalan number C(k-1)).
(define (earley-test k)
  (let ((p (make-parser '((s (a) (s s)))
                        (lambda (l) (map (lambda (x) (list x x)) l)))))
    (length (parse->trees (p (make-list k 'a)) 's 0 k))))

(define boyer-alist
  '((x f (plus (plus a b) (plus c (zero))))
    (y f (times (times a b) (plus c d)))
    (z f (reverse (append (append a b) (nil))))
    (u equal (plus a b) (difference x y))
    (w lessp (remainder a b) (member a (length b)))))

(define boyer-term
  '(implies (and (implies x y)
                 (and (implies y z)
                      (and (implies z u)
                           (implies u w))))
            (implies x w)))

(define (repeat n thunk)
  (let loop ((i 0) (r #f))
    (if (< i n) (loop (+ i 1) (thunk)) r)))

;; Avoids `read`, which would roughly double the module size.
(define (read-ints port)
  (let loop ((acc '()) (n #f))
    (let ((c (read-char port)))
      (cond ((eof-object? c) (reverse (if n (cons n acc) acc)))
            ((char<=? #\0 c #\9)
             (loop acc (+ (* 10 (or n 0)) (- (char->integer c) 48))))
            (n (loop (cons n acc) #f))
            (else (loop acc #f))))))

;; Input: earley-tokens earley-reps boyer-n boyer-reps. Keep the fallback in
;; sync with default.input.
(define params
  (guard (e (#t '(9 1 0 1)))
    (call-with-input-file "default.input" read-ints)))

(define earley-k (list-ref params 0))
(define earley-reps (list-ref params 1))
(define boyer-n (list-ref params 2))
(define boyer-reps (list-ref params 3))

;; One untimed iteration of each, so the timed ones run on a warmed-up heap as
;; in Octane's repeated iterations.
(setup-boyer)
(earley-test earley-k)
(test-boyer boyer-alist boyer-term boyer-n)

(bench-start)
(define earley-result (repeat earley-reps (lambda () (earley-test earley-k))))
(define boyer-result
  (repeat boyer-reps (lambda () (test-boyer boyer-alist boyer-term boyer-n))))
(bench-end)

(define (show label x)
  (write-string label)
  (write-string (if x (number->string x) "none"))
  (newline))
(show "earley trees: " earley-result)
(show "boyer rewrites: " boyer-result)
(flush-output-port)
