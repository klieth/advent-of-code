import Parser

namespace Data
open Parser

structure Mapping where
  dst : Nat
  src : Nat
  size : Nat
deriving Repr

structure Almanac where
  seeds : List Nat
  maps : List (List Mapping)
deriving Repr

protected abbrev Parser := SimpleParser Substring Char

protected def init : Data.Parser (List Nat) :=
  withErrorMessage "expected 'seeds:' line" do
    let _ <- Char.string "seeds:" <* Char.ASCII.whitespace
    let r <- takeMany1 (Char.ASCII.parseNat <* Char.ASCII.whitespace)
    pure r.toList

protected def mapping : Data.Parser Mapping :=
  withErrorMessage "expected range" do
    let nums <- take 3 (Char.ASCII.parseNat <* Char.ASCII.whitespace)
    -- TODO: It should be possible to prove that `take 3 <parser>` returns an
    -- array of size 3, but I don't know how to describe the proof yet. If that
    -- proof were written, we could replace `!` here.
    pure { dst := nums[0]!, src := nums[1]!, size := nums[2]! }

protected def map : Data.Parser (List Mapping) :=
  withErrorMessage "expected mapping" do
    let _ <- takeUntil (Char.char '-') anyToken
    let _ <- Char.string "to-"
    let _ <- takeUntil Char.ASCII.whitespace anyToken
    let _ <- Char.string "map:" <* Char.ASCII.whitespace
    let r <- takeMany1 Data.mapping
    pure r.toList

protected def all : Data.Parser Data.Almanac :=
  withErrorMessage "expected all" do
    let seeds <- Data.init <* Char.ASCII.whitespace
    let maps <- takeMany1 Data.map
    pure { seeds := seeds, maps := maps.toList }

protected def parse (raw : String) : Except String Data.Almanac :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def Data.Mapping.includes (r : Data.Mapping) (n : Nat) : Bool := n > r.src ∧ n < (r.src + r.size)
def Data.Mapping.apply (r : Data.Mapping) (n : Nat) : Nat := r.dst + (n - r.src)

def part1 (data : Data.Almanac) : Option Nat := (data.seeds.map getLocation).minimum? where
  getLocation (seed : Nat) : Nat := data.maps.foldl lookup seed
  lookup (acc : Nat) (mapping : List Data.Mapping) : Nat :=
    match mapping.find? (fun r => r.includes acc) with
    | .some r => r.apply acc
    | .none => acc

structure Range where
  start : Nat
  size : Nat
deriving Repr

inductive RangeApply where
  | no_overlap : RangeApply
  | m_contain_r (new : Range) : RangeApply
  | r_overlap_m_start (rem new : Range) : RangeApply
  | r_overlap_m_end (rem new : Range) : RangeApply
  | r_contain_m (rem1 rem2 new : Range) : RangeApply
deriving Repr

def Range.applyMap (r : Range) (m : Data.Mapping) : RangeApply :=
  if r.start ≥ m.src + m.size ∨ m.src ≥ r.start + r.size then
    .no_overlap
  else if r.start ≥ m.src ∧ r.start + r.size ≤ m.src + m.size then
    .m_contain_r { start := m.dst + (r.start - m.src), size := r.size }
  else if r.start < m.src ∧ r.start + r.size ≤ m.src + m.size then
    .r_overlap_m_start
      { start := r.start, size := m.src - r.start }
      { start := m.dst, size := (r.start + r.size) - m.src }
  else if r.start ≥ m.src ∧ r.start + r.size > m.src + m.size then
    let overlap := m.src + m.size - r.start
    .r_overlap_m_end
      { start := r.start + overlap, size := r.size - overlap }
      { start := m.dst + (r.start - m.src), size := overlap }
  else
    .r_contain_m
      { start := r.start, size := m.src - r.start }
      { start := m.src + m.size, size := (r.start + r.size) - (m.src + m.size) }
      { start := m.dst, size := m.size }

/-
def r : Range := { start := 3, size := 5 }

-- no overlap
-- r : 3 4 5 6 7
-- m :            8 9 10 11 => 2 3 4 5
#eval Range.applyMap r { dst := 2, src := 8, size := 4 }


-- mapping fully contains range
-- r : 3 4 5 6 7
-- m : 3 4 5 6 7     => 10 11 12 13 14
#eval Range.applyMap r { dst := 10, src := 3, size := 5 }
-- r :   3 4 5 6 7
-- m : 2 3 4 5 6 7 8 => 10 11 12 13 14 15 16
#eval Range.applyMap r { dst := 10, src := 2, size := 7 }


-- range overlaps mapping start
-- r : 3 4 5 6 7
-- m :     5 6 7 8 => 10 11 12 13
#eval Range.applyMap r { dst := 10, src := 5, size := 4 }
-- r : 3 4 5 6 7
-- m :     5 6 7   => 10 11 12
#eval Range.applyMap r { dst := 10, src := 5, size := 3 }


-- range overlaps mapping end
-- r :   3 4 5 6 7
-- m : 2 3 4 5      => 8 9 10 11
#eval Range.applyMap r { dst := 8, src := 2, size := 4 }
-- r : 3 4 5 6 7
-- m : 3 4 5      => 8 9 10
#eval Range.applyMap r { dst := 8, src := 3, size := 3 }


-- range fully contains mapping
-- r : 3 4 5 6 7
-- m :   4 5 6    => 10 11 12
#eval Range.applyMap r { dst := 10, src := 4, size := 3 }
-/

def part2 (data : Data.Almanac) : Option Nat := 
  let pairs := mkPairs data.seeds
  -- let result := data.maps.foldl (fun acc m => dbg_trace (repr acc); foldMaps acc m) pairs
  -- dbg_trace (repr result)
  let result := data.maps.foldl foldMaps pairs
  (result.map (fun r => r.start)).minimum?
  where
  mkPairs : List Nat -> List Range
    | [] => []
    | n1 :: n2 :: nx => { start := n1, size := n2 } :: mkPairs nx
    | _ => panic "unreachable"
  foldMaps (acc : List Range) (map : List Data.Mapping) : List Range :=
    match acc with
    | [] => []
    | r :: rx => (applyMaps r map) ++ (foldMaps rx map)
  applyMaps (r : Range) : List Data.Mapping -> List Range
    | [] => [r]
    | m :: mx =>
      -- dbg_trace "calling applyMap with"
      -- dbg_trace (repr r)
      -- dbg_trace (repr m)
      -- let apply_result := r.applyMap m
      -- dbg_trace (repr apply_result)
      match r.applyMap m with
      | .no_overlap => applyMaps r mx
      | .m_contain_r new => [new]
      | .r_overlap_m_start rem new => new :: applyMaps rem mx
      | .r_overlap_m_end rem new => new :: applyMaps rem mx
      | .r_contain_m rem1 rem2 new => new :: (applyMaps rem1 mx) ++ (applyMaps rem2 mx)

-- #eval part2 { seeds := [3, 5], maps := [ [{ dst := 8, src := 2, size := 4 }, { dst := 10, src := 7, size := 1 }] ] }

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (part1 parsed)
  IO.println (part2 parsed)

#eval main ["../input"]
