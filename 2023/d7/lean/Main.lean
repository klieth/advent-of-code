import Parser
import Std.Data.AssocList

namespace Data
open Parser

instance : Repr (Std.AssocList Char Nat) where
  reprPrec as n := List.repr as.toList n

def inc_or_insert (al : Std.AssocList Char Nat) (k : Char) (i : Nat) : Std.AssocList Char Nat :=
  match al with
  | .nil => .cons k i .nil
  | .cons k' v as => if k' == k then .cons k' (v + i) as else .cons k' v (inc_or_insert as k i)

structure Hand where
  cards : List Char
  card_counts : Std.AssocList Char Nat
  bid : Nat
deriving Repr

protected abbrev Parser := SimpleParser Substring Char

protected def line : Data.Parser Data.Hand :=
  withErrorMessage "expected hand" do
    let cards <- take 5 anyToken <* Char.ASCII.whitespace
    let bid <- Char.ASCII.parseNat <* Char.ASCII.whitespace
    let card_counts := cards.foldl (fun acc c => inc_or_insert acc c 1) .nil
    pure { cards := cards.toList, card_counts := card_counts, bid := bid }

protected def all : Data.Parser (List Data.Hand) :=
  withErrorMessage "expected all" do
    let lines <- takeMany1 Data.line
    pure lines.toList

protected def parse (raw : String) : Except String (List Data.Hand) :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

inductive Rank where
| five_of_a_kind
| four_of_a_kind
| full_house
| three_of_a_kind
| two_pair
| one_pair
| high_card
deriving Repr, BEq, Ord

structure RankedHand (jokers : Bool) where
  hand : Data.Hand
  rank : Rank
deriving Repr

def card_value (c : Char) (jokers : Bool) : Nat :=
  if c.isDigit then
    c.toNat - '0'.toNat
  else
    match c with
    | 'T' => 10
    | 'J' => if jokers then 0 else 11
    | 'Q' => 12
    | 'K' => 13
    | 'A' => 14
    | _ => unreachable!

instance : Ord (RankedHand j) where
  compare a b := if a.rank == b.rank then by_card_rank a.hand.cards b.hand.cards else compare b.rank a.rank where
    by_card_rank : List Char -> List Char -> Ordering
    | a :: ax, b :: bx =>
      match compare (card_value a j) (card_value b j) with
      | Ordering.eq => by_card_rank ax bx
      | o => o
    | _, _ => Ordering.eq

instance : Min (RankedHand j) where
  min x y := if compare x y == Ordering.lt then x else y

instance : Max (RankedHand j) where
  max x y := if compare x y == Ordering.gt then x else y

inductive TopGroups where
| none (jokers : Nat)
| one (a jokers : Nat)
| two (a b jokers : Nat)
deriving Repr

def top_group_sizes (al : Std.AssocList Char Nat) (jokers : Bool) : TopGroups := al.foldl go (.none 0) where
  go (acc : TopGroups) (k : Char) (v : Nat) : TopGroups :=
    if jokers ∧ k == 'J' then
      match acc with
      | .none j => .none (j + v)
      | .one a j => .one a (j + v)
      | .two a b j => .two a b (j + v)
    else
      match acc with
      | .none j => .one v j
      | .one a j => if v > a then .two v a j else .two a v j
      | .two a b j => if v > a then .two v a j else if v > b then .two a v j else .two a b j

def rank (hand : Data.Hand) (jokers : Bool) : RankedHand jokers := { hand := hand, rank := decide_rank hand } where
  decide_rank (hand : Data.Hand) : Rank :=
    match top_group_sizes hand.card_counts jokers with
    -- largest group 5
    | .one 5 _   => .five_of_a_kind
    | .none 5    => .five_of_a_kind
    -- largest group 4
    | .one 4 1   => .five_of_a_kind
    | .two 4 1 0 => .four_of_a_kind
    -- largest group 3
    | .one 3 2   => .five_of_a_kind
    | .two 3 2 0 => .full_house
    | .two 3 1 1 => .four_of_a_kind
    | .two 3 1 0 => .three_of_a_kind
    -- largest group 2
    | .one 2 3   => .five_of_a_kind
    | .two 2 2 1 => .full_house
    | .two 2 2 0 => .two_pair
    | .two 2 1 2 => .four_of_a_kind
    | .two 2 1 1 => .three_of_a_kind
    | .two 2 1 0 => .one_pair
    -- largest group 1
    | .one 1 4   => .five_of_a_kind
    | .two 1 1 3 => .four_of_a_kind
    | .two 1 1 2 => .three_of_a_kind
    | .two 1 1 1 => .one_pair
    | .two 1 1 0 => .high_card
    | c => dbg_trace ("unreachable : " ++ (repr c)); .high_card -- this should be unreachable

def List.bubblesort (ls : List α) [Ord α] [Min α] [Max α] : List α := ls.foldr swapTill [] where
  swapTill (x : α) : List α -> List α
  | [] => [x]
  | y :: ys => (min x y) :: swapTill (max x y) ys

def part1 (hands : List Data.Hand) : Nat :=
  let ranked := hands.map (rank · false)
  let sorted := ranked.bubblesort
  sorted.enum.foldl (fun acc (i, rh) => acc + ((i + 1) * rh.hand.bid)) 0

def part2 (hands : List Data.Hand) : Nat :=
  let ranked := hands.map (rank · true)
  let sorted := ranked.bubblesort
  sorted.enum.foldl (fun acc (i, rh) => acc + ((i + 1) * rh.hand.bid)) 0

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (part1 parsed)
  IO.println (part2 parsed)

#eval main ["../input"]
