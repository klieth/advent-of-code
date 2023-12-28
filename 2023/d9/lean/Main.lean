import Parser

namespace Data
open Parser

protected abbrev Parser := SimpleParser Substring Char

protected def line : Data.Parser (List Int) :=
  withErrorMessage "expected line" do
    let nums <- takeMany1 (Char.ASCII.parseInt <* optional Char.space)
    pure nums.toList

protected def all : Data.Parser (List (List Int)) :=
  withErrorMessage "expected all" do
    let lines <- takeMany1 (Data.line <* Char.ASCII.lf)
    pure lines.toList

protected def parse (raw : String) : Except String (List (List Int)) :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def differences : List Int -> List Int
| a :: b :: xs => (b - a) :: differences (b :: xs)
| _ => []

theorem differencesDecreasing : ∀ (xs : List Int) (n : Nat), (h : xs.length = .succ n) -> (differences xs).length < xs.length := by
  intro xs
  induction xs with
  | nil => intros; contradiction
  | cons head tail ih =>
    match tail with
    | .nil => simp [differences]
    | x :: xs =>
      intros
      simp [differences]
      apply Nat.succ_lt_succ
      apply ih xs.length
      simp

def findDifferences (acc : List (List Int)) (line : List Int) : List (List Int) :=
  if line.length >= 1 then
    if line.all (· == 0) then line :: acc
    else match h : line with
      | [] => []
      | _ :: _ => findDifferences (line :: acc) (differences line)
  else []
termination_by findDifferences _ l => l.length
decreasing_by apply differencesDecreasing; rw [h]; rfl

def extrapolateNext (l : List Int) : Option Int :=
  let diffs := findDifferences [] l
  diffs.foldlM (fun acc x => x.getLast?.map (· + acc)) 0

def extrapolatePrev (l : List Int) : Option Int :=
  let diffs := findDifferences [] l
  diffs.foldlM (fun acc x => x.head?.map (· - acc)) 0

def run (lines : List (List Int)) (op : List Int -> Option Int) : Option Int :=
  List.foldlM (fun acc x => x.map (· + acc)) 0 (lines.map op)

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (run parsed extrapolateNext)
  IO.println (run parsed extrapolatePrev)

#eval main ["../input"]
