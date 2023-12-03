import Lake
open Lake DSL

package «aoc»

require std from git "https://github.com/leanprover/std4" @ "v4.3.0"

@[default_target]
lean_exe «aoc» where
  root := `Main
  -- Enables the use of the Lean interpreter by the executable (e.g.,
  -- `runFrontend`) at the expense of increased binary size on Linux.
  -- Remove this line if you do not need such functionality.
  supportInterpreter := true

