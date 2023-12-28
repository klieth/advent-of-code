import Std.Data.HashMap
import Parser

namespace Data
open Parser

structure Map where
  directions : List ((String × String) -> String)
  graph : Std.HashMap String (String × String)

protected abbrev Parser := SimpleParser Substring Char

protected def graphLine : Data.Parser (String × String × String) :=
  withErrorMessage "expected graph line" do
    let getNode := take 3 anyToken
    let node <- getNode
    let _ <- Char.ASCII.whitespace *> Char.char '=' <* Char.ASCII.whitespace
    let lsucc <- Char.char '(' *> getNode
    let rsucc <- Char.string ", " *> getNode
    let _ <- Char.char ')'
    pure (node.toList.asString, lsucc.toList.asString, rsucc.toList.asString)

protected def all : Data.Parser Data.Map :=
  withErrorMessage "expected all" do
    let lr <- takeMany1 (first [Char.char 'L', Char.char 'R'])
    let _ <- takeMany Char.ASCII.whitespace
    let graphLines <- takeMany1 (Data.graphLine <* optional Char.ASCII.whitespace)
    let graph := graphLines.foldl (fun acc (node, succ) => acc.insert node succ) Std.HashMap.empty
    let lr_parse : Char -> Except String ((String × String) -> String)
      | 'L' => pure Prod.fst
      | 'R' => pure Prod.snd
      | _ => throw "unrecognized character"
    match lr.toList.mapM lr_parse with
    | .ok directions => pure { directions := directions, graph := graph }
    | .error e => withErrorMessage e throwUnexpected

protected def parse (raw : String) : Except String Data.Map :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

inductive StepsStopped
| found_end (steps : Nat)
| actual_error (err : String)

def takeStep (m : Data.Map) (acc : (String × Nat)) (d : (String × String) -> String) : Except StepsStopped (String × Nat) := do
  let takeOneStep(n : String) : Except StepsStopped String :=
    match m.graph.find? n with
    | .some n => pure (d n)
    | .none => throw (.actual_error s!"node not found in graph {n}")

  let stepped <- takeOneStep acc.fst
  if stepped.get! ⟨2⟩ == 'Z' then throw (.found_end (acc.snd + 1))
  else .ok (stepped, acc.snd + 1)

partial def wander (m : Data.Map) (start : String) : Except String Nat :=
  let rec loop (state : (String × Nat)) : Except String Nat :=
    match m.directions.foldlM (takeStep m) state with
    | .error (.found_end steps) => .ok steps
    | .error (.actual_error e) => .error e
    | .ok s => loop s
  loop (start, 0)

def part1 (m : Data.Map) : Except String Nat := wander m "AAA"

def part2 (m : Data.Map) : Except String Nat :=
  let isStartState (s : String) : Bool := s.get! ⟨2⟩ == 'A'
  let startStates := m.graph.fold (fun acc k _ => if isStartState k then k :: acc else acc) []
  let endSteps := startStates.map (wander m)
  endSteps.foldlM (fun acc n => do pure (Nat.lcm acc (<- n))) 1

-- partial: because the direction list can be traversed an infinite number of times, we can never prove that traversing this graph ever terminates.
def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (<- IO.ofExcept (part1 parsed))
  IO.println (<- IO.ofExcept (part2 parsed))

#eval main ["../test1"]
