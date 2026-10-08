# Emits trapping definitions for every unresolved "host" import.
/^  \(type \(;[0-9]+;\) \(func/ {
  n = $2; gsub(/[^0-9]/, "", n)
  sig = $0; sub(/^  \(type \(;[0-9]+;\) \(func ?/, "", sig); sub(/\)\)$/, "", sig)
  types[n] = sig
}
/^  \(import "host" / {
  name = $3; t = $0; sub(/.*\(type /, "", t); sub(/\).*/, "", t)
  print "  (func (export " name ") " types[t] " unreachable)"
}
