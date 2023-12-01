import Parser

namespace Data
open Parser

protected abbrev Parser := SimpleParser Substring Char

structure Line where
  range_start : Nat
  range_end : Nat
  chr : Char
  pwd : String
deriving Repr

protected def line : Data.Parser Line :=
  withErrorMessage "expected full line" do
    let range_start <- Char.ASCII.parseNat
    let _ <- Char.char '-'
    let range_end <- Char.ASCII.parseNat
    let _ <- Char.ASCII.whitespace
    let chr <- anyToken
    let _ <- Char.char ':' <* Char.ASCII.whitespace
    let pwd <- takeMany (Char.ASCII.alpha)
    pure { range_start := range_start, range_end := range_end, chr := chr, pwd := pwd.toList.asString }

protected def value : Data.Parser (Array Line) :=
  withErrorMessage "expected full parse" do
    takeMany1 (Data.line <* optional Char.eol)

end Data

def parse (raw : String) : Except String (Array Data.Line) :=
  match Parser.run (Data.value) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed: {e}"

def part1 (data : Array Data.Line) : Nat := data.foldl (fun acc i => if in_range i.range_start i.range_end (count i.chr i.pwd) then acc + 1 else acc) 0 where
  in_range s e n : Bool := s ≤ n ∧ n ≤ e
  count c p : Nat := p.foldl (fun acc i => if i == c then acc + 1 else acc) 0

def part2 (data : Array Data.Line) : Nat := data.foldl (fun acc i => if go i then acc + 1 else acc) 0 where
  go i' : Bool :=
    let s := i'.pwd.get ⟨i'.range_start - 1⟩
    let e := i'.pwd.get ⟨i'.range_end - 1⟩
    (s == i'.chr ∨ e == i'.chr) ∧ s != e

def main : IO Unit := do
  let raw <- IO.FS.readFile "../input"
  let data <- IO.ofExcept (parse raw)
  IO.println s!"part1: {repr (part1 data)}"
  IO.println s!"part2: {repr (part2 data)}"

#eval main
