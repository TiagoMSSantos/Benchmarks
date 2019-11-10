import strutils, algorithm, times
import TestClass

proc sameContents (words1 : var seq[string], words2 : var seq[string]): bool =
  words1.sort()
  words2.sort()
  words1 == words2

import macros

macro makeType(name, base): typed =
  template genType(tname, tbase)=
    type tname = distinct tbase
  
  result = getAst(genType(ident($name), ident($base)))

makeType("MyType", "float")
makeType("MyType2", "string")

var mt = MyType(123.4)
echo mt.float

var mt2 = MyType2("abc")
echo mt2.string

template times(integer: int, code_to_repeat: untyped): untyped =
  for i in 1..integer:
    code_to_repeat

10.times:
  #echo "This repeats 10 times, "
  let start = epochTime()
  

var
  f: File
  fname = "../input_2.txt"
  line1: TaintedString
  line2: TaintedString

if open(f, fname):
  let a = f.readLine(line1)
  let b = f.readLine(line2)
  close(f)

var words1: seq[string] = split(line1, " ")
var words2: seq[string] = split(line2, " ")

echo "words1 = ", words1, ", size = ", words1.len
echo "words2 = ", words2, ", size = ", words2.len

let start = epochTime()
let result1 = sameContents(words1, words2)
let timeInMs = (epochTime() - start) * 1000
echo "result1 = ", result1, ", time = ", timeInMs, "ms"

# let testClass2 = Dog(name: "Sparky", age: 10)
