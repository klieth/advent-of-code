/-
import Parser

namespace Data
open Parser

structure Line where
deriving Repr

protected abbrev Parser := SimpleParser Substring Char

protected def all : Data.Parser (List Data.Line) :=
  withErrorMessage "expected all" do
    throwUnexpected

protected def parse (raw : String) : Except String (List Data.Line) :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (repr parsed)

#eval main ["../test"]
-/

def main : IO Unit := IO.println "hello"

#eval main
