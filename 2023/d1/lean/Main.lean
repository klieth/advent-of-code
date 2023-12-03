import Std.Data.AssocList

def part1 (data : Array String) : Nat := data.foldl (fun acc i => acc + (value i)) 0 where
  toNat c : Nat := c.toNat - '0'.toNat
  value line : Nat :=
    let f := toNat (line.get (line.find (Char.isDigit)))
    let l := toNat (line.get (line.revFind (Char.isDigit)).get!)
    (f * 10) + l

def digits := [
  (0, "zero".toList),
  (1, "one".toList),
  (2, "two".toList),
  (3, "three".toList),
  (4, "four".toList),
  (5, "five".toList),
  (6, "six".toList),
  (7, "seven".toList),
  (8, "eight".toList),
  (9, "nine".toList)
  ].toAssocList

def findNumber (s : List Char) : Nat :=
  match s with
  | [] => panic "all strings in the input have at least one number"
  | c :: sx =>
    if Char.isDigit c then
      c.toNat - '0'.toNat
    else match digits.findEntryP? (fun _ v => v.isPrefixOf s) with
      | .some (n, _) => n
      | .none => findNumber sx

def revFindNumber (str : List Char) : Nat :=
  match go str with
  | .some n => n
  | .none => panic "all strings in the input have at least one number"
  where go s : Option Nat :=
    match s with
    | [] => .none
    | c :: sx =>
      match go sx with
      | .some n => .some n
      | .none =>
        if Char.isDigit c then
          .some (c.toNat - '0'.toNat)
        else match digits.findEntryP? (fun _ v => v.isPrefixOf s) with
          | .some (n, _) => .some n
          | .none => .none

/- look for the first letter of each word, and use substrEq? -/
def part2 (data : Array String) : Nat := data.foldl (fun acc i => acc + (value i)) 0 where
  value line : Nat :=
    ((findNumber line.toList) * 10) + (revFindNumber line.toList)

def main : IO Unit := do
  let raw <- IO.FS.lines "../input"
  IO.println (repr (part1 raw))
  IO.println (repr (part2 raw))

#eval main
