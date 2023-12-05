import Parser

namespace Data
open Parser

structure Card where
  my_numbers : Array Nat
  winning_numbers : Array Nat
deriving Repr

protected abbrev Parser := SimpleParser Substring Char

protected def line : Data.Parser (Data.Card) :=
  withErrorMessage "expected line" do
    let _ <- Char.string "Card" <* takeMany Char.ASCII.whitespace
    let _ <- Char.ASCII.parseNat
    let _ <- Char.string ": "
    let winning_numbers <- takeMany1 (takeMany Char.ASCII.whitespace *> Char.ASCII.parseNat)
    let _ <- Char.string " | "
    let my_numbers <- takeMany1 (takeMany Char.ASCII.whitespace *> Char.ASCII.parseNat)
    pure { my_numbers := my_numbers , winning_numbers := winning_numbers }

protected def all : Data.Parser (Array Data.Card) :=
  withErrorMessage "expected all" do
    takeMany1 (Data.line <* optional Char.eol)

protected def parse (raw : String) : Except String (Array Data.Card) :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def Data.Card.wins (card : Data.Card) : Nat := (card.my_numbers.filter (fun n => card.winning_numbers.any (· == n))).size

def part1 (data : Array Data.Card) : Nat := data.foldl (fun acc card => acc + (score card)) 0 where
  score (card : Data.Card) : Nat :=
    let wins := card.wins
    if wins > 0 then
      2 ^ (wins - 1)
    else 0

def part2 (data : Array Data.Card) : Nat := Prod.fst (data.foldl (fold_cards) (0, [])) where
  fold_cards (acc : Prod Nat (List Nat)) (card : Data.Card) : Prod Nat (List Nat) :=
    let wins := card.wins
    match acc.snd with
    | [] => (acc.fst + 1, add_to_n_times 1 wins [])
    | count :: rest => (acc.fst + count, add_to_n_times count wins rest)
  add_to_n_times (v count : Nat) (arr : List Nat) : List Nat :=
    match count, arr with
    | 0, _ => arr
    | n + 1, [] => (1 + v) :: add_to_n_times v n []
    | n + 1, a :: ax => (a + v) :: add_to_n_times v n ax

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (part1 parsed)
  IO.println (part2 parsed)

#eval main ["../test"]
