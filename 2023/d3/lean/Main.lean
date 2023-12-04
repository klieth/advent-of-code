structure Number where
  value : Nat
  length : Nat
  x : Nat
  y : Nat
deriving Repr

def mkNumberFromEndCoords (x y : Nat) (s : List Char) : Number :=
  let l := s.length
  { value := s.asString.toNat?.get!, length := l, x := (x - (l - 1)), y := y }

structure Symbol where
  chr : Char
  x : Nat
  y : Nat
deriving Repr

def mkSymbol (chr : Char) (x y : Nat) : Symbol :=
  ⟨ chr, x, y ⟩

structure Board where
  numbers : List Number
  symbols : List Symbol
deriving Repr

def appendNumber (n : Number) (b : Board) := { b with numbers := n :: b.numbers }
def appendSymbol (s : Symbol) (b : Board) := { b with symbols := s :: b.symbols }

def parse (raw : List Char) : Board := go raw 0 0 .none where
  go (l : List Char) (x y : Nat) (num_state : Option (List Char)) : Board :=
    match l with
    | [] => ⟨ [], [] ⟩
    | c :: lx =>
      if c.isDigit then
        match num_state with
        | .some s => go lx (x + 1) y (.some (s ++ [c]))
        | .none => go lx (x + 1) y (.some [c])
      else
        match num_state with
        | .some s => appendNumber (mkNumberFromEndCoords (x - 1) y s) (handleChar c lx x y)
        | .none => handleChar c lx x y
  handleChar (c : Char) (rest : List Char) (x y : Nat) : Board :=
    if c == '\n' then
      go rest 0 (y + 1) .none
    else if c == '.' then
      go rest (x + 1) y .none
    else appendSymbol (mkSymbol c x y) (go rest (x + 1) y .none)

def adjacent (n : Number) (s : Symbol) : Bool :=
  (n.x - 1 ≤ s.x ∧ s.x ≤ n.x + n.length) ∧ -- x
    (n.y - 1 ≤ s.y ∧ s.y ≤ n.y + 1) -- y

def part1 (board : Board) : Nat := board.numbers.foldl (fun acc number => acc + if anyAdjacent number board.symbols then number.value else 0) 0 where
  anyAdjacent (n : Number) (ss : List Symbol) : Bool := ss.any (adjacent n)

def part2 (board : Board) : Nat := board.symbols.filterMap ((maybeGearRatio? board.numbers) ∘ maybeGear?) |> List.foldl (· + ·) 0 where
  maybeGear? (s : Symbol) : Option Symbol := if s.chr == '*' then .some s else .none
  maybeGearRatio? (numbers : List Number) (os : Option Symbol) : Option Nat :=
    os >>= (gearRatio? numbers)
  gearRatio? (numbers: List Number) (s : Symbol) : Option Nat :=
    let n := numbers.filter (adjacent · s)
    if n.length == 2 then
      .some (n.foldl (fun acc n => acc * n.value) 1)
    else
      .none

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed := parse raw.toList
  IO.println (part1 parsed)
  IO.println (part2 parsed)
