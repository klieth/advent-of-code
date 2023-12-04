import Parser

namespace Data
open Parser

structure Handful where
  blue : Nat
  green : Nat
  red : Nat
deriving Repr

structure Game where
  game_num : Nat
  handfuls : Array Handful
deriving Repr

protected abbrev Parser := SimpleParser Substring Char

protected def handful : Data.Parser Handful := do
  let raw <- sepBy1 (Char.char ',') do
    let amt <- (optional Char.ASCII.whitespace) *> Char.ASCII.parseNat
    let color <- Char.ASCII.whitespace *> first [Char.string "blue", Char.string "green", Char.string "red"]
    pure (color, amt)

  let rec find (r : List (Prod String Nat)) (c : String) : Nat :=
    match r with
    | [] => 0
    | (c', n) :: rx => if c == c' then n else find rx c

  let rawList := raw.toList
  pure { blue := (find rawList "blue"), green := (find rawList "green"), red := (find rawList "red") }

protected def game : Data.Parser Game :=
  withErrorMessage "expected full game" do
    let _ <- Char.string "Game "
    let game_num <- Char.ASCII.parseNat
    let _ <- Char.chars ": "
    let handfuls <- sepBy1 (Char.char ';') do
      (optional Char.ASCII.whitespace) *> Data.handful
    pure { game_num := game_num, handfuls := handfuls }

protected def all : Data.Parser (Array Game) :=
  withErrorMessage "expected all" do
    takeMany1 (Data.game <* optional Char.eol)

protected def parse (raw: String) : Except String (Array Data.Game) :=
  -- TODO Parser.endOfInput
  match Parser.run (Data.all) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def part1 (games : Array Data.Game) : Nat :=
  let withinLimits handful : Bool := handful.red ≤ 12 ∧ handful.green ≤ 13 ∧ handful.blue ≤ 14
  let valid g : Option Nat := if g.handfuls.all (withinLimits ·) then .some g.game_num else .none
  games.foldl (fun acc game => acc + (valid game).getD 0) 0

def part2 (games : Array Data.Game) : Nat :=
  let rec maximums (handfuls : List Data.Handful) (acc : Data.Handful) : Data.Handful := 
    match handfuls with
    | [] => acc
    | h :: hx => maximums hx { blue := max h.blue acc.blue, green := max h.green acc.green, red := max h.red acc.red }
  let power g : Nat :=
    let result := maximums g.handfuls.toList { blue := 0, green := 0, red := 0 }
    result.blue * result.red * result.green
  games.foldl (fun acc game => acc + (power game)) 0

def main : IO Unit := do
  let raw <- IO.FS.readFile "../input"
  let data <- IO.ofExcept (Data.parse raw)
  IO.println (part1 data)
  IO.println (part2 data)

#eval main
